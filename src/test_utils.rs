pub fn test_assert(value: bool, message: ~str) {
    if !value {
        fail!(message);
    }
}
