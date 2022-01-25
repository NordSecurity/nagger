// Copyright 2023 Nord Security
pub fn main() {}

// Dummy function to get a result
fn get_array() -> [i32; 4] {
    [1, 2, 3, 4]
}

// This function should trigger the warning
fn test_index_access() {
    let _result = get_array()[2];
}

// This function should pass
fn test_index_access_slice_full_range() {
    let _result = &get_array()[..];
}

// This function should trigger the warning
fn test_index_access_range() {
    let _result = &get_array()[1..2];
}

// This function should trigger the warning
fn test_index_access_range_from() {
    let _result = &get_array()[1..];
}

// This function should trigger the warning
fn test_index_access_range_from_inclusive() {
    let _result = &get_array()[..=2];
}

// This should not as it is in a test function
#[cfg(test)]
fn test_index_access_in_test() {
    let _result = get_array()[2];
}

// This should not as it is in a test function
#[test]
fn test_index_access_in_cfg_test() {
    let _result = get_array()[2];
}

// This function should trigger the warning
#[cfg(not(test))]
fn test_index_access_in_test() {
    let _result = get_array()[2];
}

// This function should pass, as we allow the usage of [] here
#[allow(index_access_check)]
fn test_allowed_index_access() {
    let _result = get_array()[2];
}
