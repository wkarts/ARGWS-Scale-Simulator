"""Testes sem hardware. python tests/check_toledo8540.py --adp-parser PATH [--rustc PATH]."""
import argparse, pathlib, subprocess, tempfile, json, math
p = argparse.ArgumentParser()
p.add_argument('--adp-parser', required=True, type=pathlib.Path)
p.add_argument('--rustc', default='rustc')
a = p.parse_args()
root = pathlib.Path(__file__).resolve().parents[1]
s = (root / 'src-tauri/src/commands/scale_simulator.rs').read_text()
frame = s[s.index('fn build_frame('):s.index('fn status_from_runtime(')]
legacy = (root / 'tests/fixtures/legacy_frames.rs').read_text()
encoder = root / 'src-tauri/src/commands/toledo8540_encoder.rs'
prelude = f'#[path={json.dumps(str(encoder))}] mod toledo8540_encoder;\n#[path={json.dumps(str(a.adp_parser.resolve()))}] mod adp;\n'
prelude += '''
#[derive(Clone, Copy)]
enum ScaleSimulatorDriver { Wt1000n, Wt27r, Wt3000ir, Toledo8540, ToledoPrix3, A12e, LiderLd2052, Filizola, GenericAscii }
struct ScaleSimulatorConfig { driver: ScaleSimulatorDriver, unit: String, capacity_kg:f64, division_kg:f64 }
struct ScaleState { gross_kg:f64, tare_kg:f64, stable:bool, gross_mode:bool }
fn config() -> ScaleSimulatorConfig { ScaleSimulatorConfig { driver:ScaleSimulatorDriver::Toledo8540, unit:"kg".into(), capacity_kg:80000.0, division_kg:1.0 } }
fn state() -> ScaleState { ScaleState { gross_kg:1234.0, tare_kg:0.0, stable:true, gross_mode:true } }
fn close(a:f64,b:f64) { assert!((a-b).abs()<1e-8,"{a} != {b}"); }
'''
tests = r'''
#[test] fn other_models_byte_identical() {
 for driver in [ScaleSimulatorDriver::Wt1000n,ScaleSimulatorDriver::Wt27r,ScaleSimulatorDriver::Wt3000ir,ScaleSimulatorDriver::ToledoPrix3,ScaleSimulatorDriver::A12e,ScaleSimulatorDriver::LiderLd2052,ScaleSimulatorDriver::Filizola,ScaleSimulatorDriver::GenericAscii] {
  for weight in [-1234.567,0.0,1234.567,999999.0] { for tare in [0.0,2000.25] { for stable in [true,false] { for gross_mode in [true,false] { for unit in ["kg","lb"] {
   let mut c=config(); c.driver=driver;c.unit=unit.into();
   let s=ScaleState{gross_kg:weight,tare_kg:tare,stable,gross_mode};
   assert_eq!(build_frame(&c,&s).as_bytes(),legacy_build_frame(&c,&s).as_bytes());
  }}}}}
 }
}
#[test] fn end_to_end_decimal_sign_tare_motion_unit() {
 for decimal in 0..=7 { for step in [1.0,2.0,5.0] {
  let factor=10_f64.powi(2-decimal);
  let mut c=config();c.division_kg=step*factor;c.capacity_kg=999999.0*factor;
  for gross_mode in [true,false] {for stable in [true,false] {for sign in [-1.0,1.0] {
   let s=ScaleState{gross_kg:sign*1234.0*factor,tare_kg:200.0*factor,stable,gross_mode};
   let raw=build_frame(&c,&s);assert_eq!(raw.len(),18);assert!(raw.is_ascii());
   let value=adp::parse(raw.as_bytes()).unwrap();
   close(value.gross,s.gross_kg);close(value.tare,s.tare_kg);close(value.net,s.gross_kg-s.tare_kg);assert_eq!(value.stable,stable);assert_eq!(value.unit,"kg");
  }}}
 }}
 let mut c=config(); c.unit="lb".into();c.division_kg=0.01;c.capacity_kg=100.0;
 let s=ScaleState{gross_kg:4.5359237,tare_kg:0.90718474,stable:true,gross_mode:false};
 let raw=build_frame(&c,&s);let value=adp::parse(raw.as_bytes()).unwrap();
 close(value.gross,10.0);close(value.tare,2.0);close(value.net,8.0);assert_eq!(value.unit,"lb");
}
#[test] fn zero_overload_invalid_and_boundaries() {
 let c=config();let mut s=state();s.gross_kg=0.0;let raw=build_frame(&c,&s);assert_eq!(raw.as_bytes()[17],119);close(adp::parse(raw.as_bytes()).unwrap().gross,0.0);
 for weight in [80001.0,-80001.0,f64::NAN,f64::INFINITY] {s.gross_kg=weight;let raw=build_frame(&c,&s);assert_eq!(raw.len(),18);assert_eq!(raw.as_bytes()[2]&4,4);assert!(adp::parse(raw.as_bytes()).is_err());}
 s.gross_kg=1.0;s.tare_kg=1000000.0;assert!(adp::parse(build_frame(&c,&s).as_bytes()).is_err());
 assert!(toledo8540_encoder::settings(1.0,100.0,"g").is_err());
 assert!(toledo8540_encoder::settings(0.001,80000.0,"kg").is_err());
 assert!(toledo8540_encoder::settings(0.3,100.0,"kg").is_err());
 assert!(toledo8540_encoder::settings(f64::NAN,100.0,"kg").is_err());
}
#[test] fn checksum_control_values_all_chunks_and_corruption() {
 let mut c=config();c.capacity_kg=999999.0;let mut s=state();s.tare_kg=999999.0;s.gross_mode=false;s.stable=false;
 for target in [0,2,10,13] {
  let raw=(0..=999999).find_map(|w|{s.gross_kg=999999.0-w as f64;let frame=build_frame(&c,&s);if frame.as_bytes()[17]==target{Some(frame.into_bytes())}else{None}}).unwrap();
  assert!(adp::parse(&raw).is_ok());let wire=[raw.clone(),raw.clone()].concat();
  for cut in 0..=wire.len(){let mut buffer=wire[..cut].to_vec();let mut frames=adp::split_frames(&mut buffer);buffer.extend_from_slice(&wire[cut..]);frames.extend(adp::split_frames(&mut buffer));assert_eq!(frames,vec![raw.clone(),raw.clone()]);assert!(buffer.is_empty());}
  for bit in 0..7 {let mut bad=raw.clone();bad[17]^=1<<bit;assert!(adp::parse(&bad).is_err());}
 }
}
'''
# Rust producer, Python independent consumer: transfer real bytes as hex.
main = r'''
fn main() {
 for unit in ["kg","lb"] {for negative in [false,true] {for gross_mode in [true,false] {for stable in [true,false] {
  let mut c=config();c.unit=unit.into();c.capacity_kg=100.0;c.division_kg=0.001;
  let mut s=state();s.gross_kg=if negative {-12.345}else{12.345};s.tare_kg=2.0;s.gross_mode=gross_mode;s.stable=stable;
  let raw=build_frame(&c,&s);let hex:String=raw.as_bytes().iter().map(|b|format!("{b:02x}")).collect();
  println!("{} {} {} {} {}",hex,s.gross_kg,s.tare_kg,gross_mode,stable);
 }}}}
}
'''
with tempfile.TemporaryDirectory(prefix='simulator8540-') as temp:
    temp=pathlib.Path(temp); src=temp/'harness.rs';src.write_text(prelude+frame+legacy+tests+main)
    binary=temp/'tests'
    subprocess.run([a.rustc,'--edition=2021','--test',str(src),'-o',str(binary)],check=True)
    subprocess.run([str(binary),'--nocapture'],check=True)
    producer=temp/'producer'
    subprocess.run([a.rustc,'--edition=2021',str(src),'-o',str(producer)],check=True)
    lines=subprocess.check_output([str(producer)],text=True).splitlines()
    for line in lines:
        wire,gross,tare,gross_mode,stable=line.split();raw=bytes.fromhex(wire)
        assert len(raw)==18 and raw[0]==2 and raw[16]==13 and sum(raw)%128==0
        factor=10**(2-(raw[1]&7));value=int(raw[4:10])*factor*(-1 if raw[2]&2 else 1)
        decoded_tare=int(raw[10:16])*factor
        kg_per_unit=1 if raw[2]&16 else 0.45359237
        expected=(float(gross) if gross_mode=='true' else float(gross)-float(tare))/kg_per_unit
        assert abs(value-expected)<=factor/2+1e-8
        assert abs(decoded_tare-float(tare)/kg_per_unit)<=factor/2+1e-8
        assert (raw[2]&1==0)==(gross_mode=='true')
        assert (raw[2]&8==0)==(stable=='true')
    print(f'Python independent decoder: {len(lines)} real Rust frames passed.')
