pub(crate) fn parse_value(s: &str) -> Option<i128> {
    if s.starts_with("0x") {
        let stripped = s.strip_prefix("0x").unwrap();
        return Some(i128::from_str_radix(stripped, 16).unwrap());
    } else if s.starts_with("0b") {
        let stripped = s.strip_prefix("0b").unwrap();
        return Some(i128::from_str_radix(stripped, 2).unwrap());
    } else if s.contains('\"') {
        let string = s.replace('\"', "").replace(r"\0", "\0");
        let stripped = string.as_bytes();
        let mut bytes = [0_u8; 8];
        let len = bytes.len() - stripped.len();
        bytes[len..].clone_from_slice(stripped);

        let value = u64::from_be_bytes(bytes);

        return Some(value.into());
    }
    let v = str::parse(s);
    if let Ok(v) = v {
        return Some(v);
    }

    None
}
