// Copyright 2023 Nord Security
pub fn main() {}

// Dummy function to get a result
fn get_result() -> Result<u32, u32> {
    Ok((0))
}

// This function should trigger the warning
fn test_unwrap() {
    let _result = get_result().unwrap();
}

// This should not as it is in a test function
#[cfg(test)]
fn test_unwrap_in_test() {
    let _result = get_result().unwrap();
}

// This function should pass, as we allow the usage of unwrap here
#[allow(unwrap_check)]
fn test_allowed_unwrap() {
    let _result = get_result().unwrap();
}
