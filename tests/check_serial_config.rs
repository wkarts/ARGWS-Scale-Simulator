// Included after the real module in the isolated validation harness.
#[test]
fn preserves_other_drivers_and_configures_toledo() {
    let mut c: ScaleSimulatorConfig = serde_json::from_value(serde_json::json!({
        "id":null,"name":"test","driver":"toledo8540","mode":"tcp_server",
        "host":"127.0.0.1","port":9103,"serial_port":" COM10 ","baud_rate":4800,
        "capacity_kg":80000.0,"division_kg":1.0,"initial_weight_kg":1234.0,
        "variation_kg":0.0,"interval_ms":500,"stable":true,"zero_tracking":true,"unit":"kg"
    })).unwrap();
    assert_eq!(c.toledo8540_stop_bits, None);
    let expected = serialport::new("COM10", 4800).timeout(Duration::from_millis(250))
        .data_bits(serialport::DataBits::Seven).parity(serialport::Parity::Even);
    assert_eq!(serial_builder(&c), expected.clone().stop_bits(serialport::StopBits::One));
    c.toledo8540_stop_bits = Some(2);
    assert_eq!(serial_builder(&c), expected.stop_bits(serialport::StopBits::Two));
    c.toledo8540_stop_bits = Some(3);
    assert!(validate_config(&c).is_err());
    for driver in [ScaleSimulatorDriver::Wt1000n,ScaleSimulatorDriver::Wt27r,
        ScaleSimulatorDriver::Wt3000ir,ScaleSimulatorDriver::ToledoPrix3,
        ScaleSimulatorDriver::A12e,ScaleSimulatorDriver::LiderLd2052,
        ScaleSimulatorDriver::Filizola,ScaleSimulatorDriver::GenericAscii] {
        c.driver = driver;
        c.baud_rate = 9600;
        assert_eq!(serial_builder(&c), serialport::new("COM10",9600).timeout(Duration::from_millis(250)));
        assert!(validate_config(&c).is_ok());
    }
}

#[tokio::test]
async fn tcp_transport_preserves_all_18_bytes_including_nul_checksum() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    let c: ScaleSimulatorConfig = serde_json::from_value(serde_json::json!({
        "id":null,"name":"loopback-test","driver":"toledo8540","mode":"tcp_server",
        "host":"127.0.0.1","port":port,"serial_port":"unused","baud_rate":4800,
        "capacity_kg":999999.0,"division_kg":1.0,"initial_weight_kg":0.0,
        "variation_kg":0.0,"interval_ms":50,"stable":false,"zero_tracking":false,"unit":"kg"
    })).unwrap();
    let mut s = ScaleState {gross_kg:0.0,tare_kg:999999.0,stable:false,gross_mode:false,last_frame:String::new()};
    let expected = (0..=999999).find_map(|weight| {
        s.gross_kg = 999999.0 - f64::from(weight);
        let frame = build_frame(&c, &s);
        (frame.as_bytes()[17] == 0).then_some(frame)
    }).unwrap();
    let state = Arc::new(Mutex::new(s));
    let clients = Arc::new(AtomicUsize::new(0));
    let shutdown = Arc::new(AtomicBool::new(false));
    let logs = Arc::new(Mutex::new(VecDeque::new()));
    let task = tokio::spawn(run_tcp_server(listener,c,state,clients,shutdown.clone(),logs));
    let mut client = TcpStream::connect(("127.0.0.1",port)).await.unwrap();
    let mut bytes = [0_u8;36];
    tokio::time::timeout(Duration::from_secs(3),client.read_exact(&mut bytes)).await.unwrap().unwrap();
    assert_eq!(&bytes[..18],expected.as_bytes());
    assert_eq!(&bytes[18..],expected.as_bytes());
    assert_eq!(bytes[17],0);
    shutdown.store(true,Ordering::SeqCst);
    drop(client);
    tokio::time::timeout(Duration::from_secs(3),task).await.unwrap().unwrap();
}

#[cfg(test)]
fn advanced_config() -> ScaleSimulatorConfig {
    serde_json::from_value(serde_json::json!({
        "id":null,"name":"options-test","driver":"toledo8540","mode":"tcp_server",
        "host":"127.0.0.1","port":9103,"serial_port":"COM10","baud_rate":4800,
        "capacity_kg":80000.0,"division_kg":1.0,"initial_weight_kg":1234.0,
        "variation_kg":0.0,"interval_ms":50,"stable":true,"zero_tracking":false,"unit":"kg"
    })).unwrap()
}

#[test]
fn explicit_serial_options_override_defaults_without_changing_frames() {
    let mut c = advanced_config();
    for driver in [ScaleSimulatorDriver::Toledo8540, ScaleSimulatorDriver::Wt1000n] {
        c.driver = driver;
        for (bits, actual_bits) in [(5,serialport::DataBits::Five),(6,serialport::DataBits::Six),(7,serialport::DataBits::Seven),(8,serialport::DataBits::Eight)] {
            for (parity, actual_parity) in [("none",serialport::Parity::None),("even",serialport::Parity::Even),("odd",serialport::Parity::Odd)] {
                for (stop, actual_stop) in [(1,serialport::StopBits::One),(2,serialport::StopBits::Two)] {
                    c.serial_data_bits=Some(bits);c.serial_parity=Some(parity.into());c.serial_stop_bits=Some(stop);
                    assert!(validate_config(&c).is_ok());
                    assert_eq!(serial_builder(&c),serialport::new("COM10",4800).timeout(Duration::from_millis(250)).data_bits(actual_bits).parity(actual_parity).stop_bits(actual_stop));
                }
            }
        }
    }
    c.serial_parity=Some("mark".into());assert!(validate_config(&c).is_err());
    c.serial_parity=None;c.serial_data_bits=Some(9);assert!(validate_config(&c).is_err());
    c.serial_data_bits=None;c.serial_stop_bits=Some(3);assert!(validate_config(&c).is_err());
}

#[test]
fn advanced_frame_changes_are_explicit_and_default_is_byte_identical() {
    let mut c=advanced_config();
    let s=ScaleState {gross_kg:1234.0,tare_kg:0.0,stable:true,gross_mode:true,last_frame:String::new()};
    for driver in [ScaleSimulatorDriver::Wt1000n,ScaleSimulatorDriver::Wt27r,ScaleSimulatorDriver::Wt3000ir,ScaleSimulatorDriver::Toledo8540,ScaleSimulatorDriver::ToledoPrix3,ScaleSimulatorDriver::A12e,ScaleSimulatorDriver::LiderLd2052,ScaleSimulatorDriver::Filizola,ScaleSimulatorDriver::GenericAscii] {
        c.driver=driver;
        c.frame_override_enabled=false;c.frame_prefix_hex=Some("invalid".into());
        assert_eq!(transmission_frame(&c,&s).unwrap(),build_frame(&c,&s));
        c.frame_override_enabled=true;c.frame_prefix_hex=None;
        assert_eq!(transmission_frame(&c,&s).unwrap(),build_frame(&c,&s));
        c.frame_prefix_hex=Some("02".into());c.frame_suffix_hex=Some("0D 0A".into());
        let raw=transmission_frame(&c,&s).unwrap();assert_eq!(raw.as_bytes()[0],2);
        if matches!(c.driver,ScaleSimulatorDriver::Toledo8540) {assert_eq!(&raw.as_bytes()[raw.len()-3..raw.len()-1],b"\r\n");assert_eq!(raw.as_bytes().iter().map(|b|u32::from(*b)).sum::<u32>()%128,0);}else{assert!(raw.ends_with("\r\n"));}
        c.frame_prefix_hex=None;c.frame_suffix_hex=None;
    }
    c.driver=ScaleSimulatorDriver::Toledo8540;
    let normal=build_frame(&c,&s);
    c.frame_checksum_mode=Some("corrupt".into());let corrupt=transmission_frame(&c,&s).unwrap();assert_eq!(corrupt.len(),18);assert_eq!(corrupt.as_bytes()[17],normal.as_bytes()[17]^1);
    c.frame_checksum_mode=Some("omit".into());assert_eq!(transmission_frame(&c,&s).unwrap().as_bytes(),&normal.as_bytes()[..17]);
    c.frame_checksum_mode=None;c.frame_prefix_hex=Some("".into());c.frame_suffix_hex=Some("".into());assert_eq!(transmission_frame(&c,&s).unwrap().len(),16);
    for invalid in ["GG","0","+0","80","020202020202020202","é"] {c.frame_prefix_hex=Some(invalid.into());assert!(validate_config(&c).is_err());}
    c.frame_prefix_hex=None;c.frame_suffix_hex=None;c.frame_checksum_mode=Some("omit".into());c.driver=ScaleSimulatorDriver::Wt1000n;assert!(validate_config(&c).is_err());
}

#[test]
fn existing_oscillation_stays_quantized_and_inside_capacity() {
    let mut c=advanced_config();c.capacity_kg=100.0;c.division_kg=0.5;c.variation_kg=2.0;c.stable=false;
    let state=Arc::new(Mutex::new(ScaleState{gross_kg:99.0,tare_kg:3.0,stable:true,gross_mode:false,last_frame:String::new()}));
    for _ in 0..100 {
        apply_variation(&c,&state);
        let s=state.lock().unwrap();
        assert!(s.gross_kg.is_finite() && s.gross_kg.abs()<=100.0);
        assert_eq!((s.gross_kg/c.division_kg).fract(),0.0);
        assert_eq!(s.tare_kg,3.0);
        assert!(!s.stable);
    }
}
