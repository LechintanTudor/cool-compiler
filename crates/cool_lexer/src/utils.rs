pub fn is_valid_ident(str: &str) -> bool {
    match str.len() {
        0 => false,
        1 => {
            let first_char = str.chars().next().unwrap_or(' ');
            unicode_ident::is_xid_start(first_char)
        }
        _ => {
            let mut char_iter = str.chars();
            let first_char = char_iter.next().unwrap_or(' ');

            if !(unicode_ident::is_xid_start(first_char) || first_char == '_') {
                return false;
            }

            char_iter.all(unicode_ident::is_xid_continue)
        }
    }
}
