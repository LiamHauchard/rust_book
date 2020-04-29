pub fn _add_one(num: Option<i32>) -> Option<i32> {
    match num {
        None => None,
        Some(val) => Some(val + 1),
    }
}
