// Gerador original v0.1.9, fixture de regressão somente.
fn legacy_build_frame(config: &ScaleSimulatorConfig, state: &ScaleState) -> String {
    let net_kg = state.gross_kg - state.tare_kg;
    let display_kg = if state.gross_mode { state.gross_kg } else { net_kg };
    let sign = if display_kg < 0.0 { '-' } else { '+' };
    let unit = config.unit.trim();
    let scaled = display_kg.abs().round() as i64;
    let gross_scaled = state.gross_kg.abs().round() as i64;
    let tare_scaled = state.tare_kg.abs().round() as i64;
    let mode = if state.gross_mode { "GS" } else { "NT" };
    let stability = if state.stable { "ST" } else { "US" };

    match config.driver {
        // Compatível com o parser do All Driver Platform:
        // parts[0] == "0" significa estável; parts[1]=bruto; parts[2]=tara; parts[3]=líquido.
        ScaleSimulatorDriver::Wt1000n => {
            let status = if state.stable { "0" } else { "1" };
            format!("{status},{},{},{}\r\n", signed_value(state.gross_kg), signed_value(state.tare_kg), signed_value(net_kg))
        }
        ScaleSimulatorDriver::Wt27r => format!("={sign}{scaled:06}{unit}\r\n"),
        // Compatível com o parser WT3000IR do ADP: status em parts[0] e números em parts[2].
        ScaleSimulatorDriver::Wt3000ir => {
            format!("{stability},{mode},{sign}{scaled:06}{unit} {tare_scaled:06}{unit}\r\n")
        }
        // Compatível com parser Toledo 8540 do ADP: STX, byte de status na posição 2,
        // bruto em 4..10 e tara em 10..16.
        ScaleSimulatorDriver::Toledo8540 => {
            let status_byte = if state.stable { '\u{0000}' } else { '\u{0004}' };
            format!("\u{0002} {status_byte} {gross_scaled:06}{tare_scaled:06}\r\n")
        }
        ScaleSimulatorDriver::ToledoPrix3 => format!("\u{0002}{sign}{scaled:06}{unit}\u{0003}"),
        // Compatível com parser A12e do ADP.
        ScaleSimulatorDriver::A12e => format!("wn{sign}{scaled:06}{unit}\r\n"),
        // Compatível com parser LIDERLD2052 do ADP: 0x01 + 7 ASCII + espaço + E/I + 0x02.
        ScaleSimulatorDriver::LiderLd2052 => {
            let status = if state.stable { 'E' } else { 'I' };
            format!("\u{0001}{scaled:>07} {status}\u{0002}")
        }
        ScaleSimulatorDriver::Filizola => format!("P{sign}{scaled:06}{unit}\r\n"),
        ScaleSimulatorDriver::GenericAscii => format!("{sign}{scaled:06} {unit}\r\n"),
    }
}

