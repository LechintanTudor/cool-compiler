pub fn is_identifier_middle_part(byte: u8) -> bool {
    byte == b'_' || byte.is_ascii_alphanumeric()
}
