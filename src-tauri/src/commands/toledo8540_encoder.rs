//! Toledo 8540 contínuo analógico/numérico (C44A=1).
//! https://devser.com.br/downloads/protocolo-toledo-8540.pdf
//! Apenas bytes de 7 bits: String preserva inclusive NUL/STX/CR no checksum.

pub(super) fn settings(
    division_kg: f64,
    capacity_kg: f64,
    unit: &str,
) -> Result<(u8, f64), String> {
    if !division_kg.is_finite()
        || division_kg <= 0.0
        || !capacity_kg.is_finite()
        || capacity_kg <= 0.0
    {
        return Err("Toledo 8540: divisão e capacidade devem ser finitas e positivas.".into());
    }
    match unit.trim() {
        "kg" => {
            for decimal in 0..=7 {
                let factor = 10_f64.powi(2 - decimal);
                let increment = division_kg / factor;
                for (step, bits) in [(1.0, 0x08), (2.0, 0x10), (5.0, 0x18)] {
                    if (increment - step).abs() < 1e-7 {
                        if (capacity_kg / factor).round() > 999999.0 {
                            return Err(
                                "Toledo 8540: capacidade/divisão excede o campo de seis dígitos."
                                    .into(),
                            );
                        }
                        return Ok((0x60 | bits | decimal as u8, factor));
                    }
                }
            }
            Err(
                "Toledo 8540: use divisão 1, 2 ou 5 vezes potência de 10 (0,00001 a 500 kg)."
                    .into(),
            )
        }
        "lb" => {
            // A tela e o estado continuam em kg. A transmissão converte para lb.
            // Incremento 1; escolhe a melhor precisão que cabe na capacidade,
            // limitada a um décimo da divisão física e cinco casas decimais.
            let division_lb = division_kg / 0.45359237;
            for decimal in (0..=7).rev() {
                let factor = 10_f64.powi(2 - decimal);
                if (capacity_kg / 0.45359237 / factor).round() <= 999999.0
                    && (factor >= division_lb / 10.0 || decimal == 7)
                {
                    return Ok((0x68 | decimal as u8, factor));
                }
            }
            Err("Toledo 8540: capacidade em lb excede seis dígitos.".into())
        }
        _ => Err("Toledo 8540 suporta somente kg ou lb.".into()),
    }
}

pub(super) fn encode(
    gross_kg: f64,
    tare_kg: f64,
    stable: bool,
    gross_mode: bool,
    capacity_kg: f64,
    division_kg: f64,
    unit: &str,
) -> String {
    let config = settings(division_kg, capacity_kg, unit);
    let (swa, factor) = config.as_ref().copied().unwrap_or((0x6a, 1.0));
    let conversion = if unit.trim() == "lb" {
        1.0 / 0.45359237
    } else {
        1.0
    };
    let displayed = if gross_mode {
        gross_kg
    } else {
        gross_kg - tare_kg
    };
    let weight = (displayed.abs() * conversion / factor).round();
    let tare = (tare_kg * conversion / factor).round();
    let overload = config.is_err()
        || !gross_kg.is_finite()
        || !tare_kg.is_finite()
        || gross_kg.abs() > capacity_kg
        || tare_kg < 0.0
        || !weight.is_finite()
        || weight > 999999.0
        || !tare.is_finite()
        || tare > 999999.0;
    let swb = 0x20
        | u8::from(!gross_mode)
        | (u8::from(displayed < 0.0) << 1)
        | (u8::from(overload) << 2)
        | (u8::from(!stable) << 3)
        | if unit.trim() == "kg" { 0x10 } else { 0 };
    // Overload never truncates fields or silently emits a valid zero reading.
    let (weight, tare) = if overload {
        (0, 0)
    } else {
        (weight as u32, tare as u32)
    };
    let mut bytes = vec![0x02, swa, swb, 0x20];
    bytes.extend_from_slice(format!("{weight:06}{tare:06}\r").as_bytes());
    let sum = bytes.iter().fold(0_u8, |sum, byte| sum.wrapping_add(*byte));
    bytes.push(sum.wrapping_neg() & 0x7f);
    // All fields and checksum are ASCII; no lossy UTF-8 conversion.
    bytes.into_iter().map(char::from).collect()
}
