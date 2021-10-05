pub fn if_some(b: bool) -> Option<bool> {
    if b {
        Some(b)
    } else {
        None
    }
}
