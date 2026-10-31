//! Explicit fault-injection framing options. Disabled by default; never changes
//! a protocol encoder. Bytes are ASCII to preserve the application's String wire path.

#[derive(Clone, Copy)]
pub(super) enum Kind {
    Toledo8540,
    Lider,
    StxEtx,
    Line,
}

fn hex(value: &str) -> Result<Vec<u8>, String> {
    let compact: String = value
        .chars()
        .filter(|ch| !ch.is_ascii_whitespace())
        .collect();
    if compact.len() > 16
        || compact.len() & 1 != 0
        || !compact.bytes().all(|byte| byte.is_ascii_hexdigit())
    {
        return Err("Frame de teste: informe até 8 bytes hexadecimais (ex.: 02 ou 0D 0A).".into());
    }
    compact
        .as_bytes()
        .chunks(2)
        .map(|pair| {
            let text = std::str::from_utf8(pair).map_err(|_| "Hex inválido".to_string())?;
            let byte = u8::from_str_radix(text, 16)
                .map_err(|_| "Byte hexadecimal inválido.".to_string())?;
            if byte > 0x7f {
                Err("Frame de teste aceita bytes 00–7F; transporte atual é ASCII de 7 bits.".into())
            } else {
                Ok(byte)
            }
        })
        .collect()
}

pub(super) fn validate(
    kind: Kind,
    prefix: Option<&str>,
    suffix: Option<&str>,
    checksum: &str,
) -> Result<(), String> {
    if let Some(value) = prefix {
        hex(value)?;
    }
    if let Some(value) = suffix {
        hex(value)?;
    }
    if !matches!(checksum, "protocol" | "corrupt" | "omit") {
        return Err("Modo de checksum de teste inválido.".into());
    }
    if !matches!(kind, Kind::Toledo8540) && checksum != "protocol" {
        return Err("Alteração de checksum de teste disponível apenas para Toledo 8540.".into());
    }
    Ok(())
}

pub(super) fn apply(
    frame: &str,
    kind: Kind,
    prefix: Option<&str>,
    suffix: Option<&str>,
    checksum: &str,
) -> Result<String, String> {
    validate(kind, prefix, suffix, checksum)?;
    let raw = frame.as_bytes();
    let (head, tail, cks) = match kind {
        Kind::Toledo8540 => (1, 1, 1),
        Kind::Lider | Kind::StxEtx => (1, 1, 0),
        Kind::Line => (0, 2, 0),
    };
    if raw.len() < head + tail + cks {
        return Err("Frame original incompleto.".into());
    }
    let prefix = match prefix {
        Some(value) => hex(value)?,
        None => raw[..head].to_vec(),
    };
    let suffix = match suffix {
        Some(value) => hex(value)?,
        None => raw[raw.len() - tail - cks..raw.len() - cks].to_vec(),
    };
    let mut bytes = prefix;
    bytes.extend_from_slice(&raw[head..raw.len() - tail - cks]);
    bytes.extend_from_slice(&suffix);
    if cks == 1 && checksum != "omit" {
        let mut value = bytes
            .iter()
            .fold(0_u8, |sum, byte| sum.wrapping_add(*byte))
            .wrapping_neg()
            & 0x7f;
        if checksum == "corrupt" {
            value ^= 1;
        }
        bytes.push(value);
    }
    String::from_utf8(bytes).map_err(|_| "Frame de teste não é UTF-8 válido.".into())
}
