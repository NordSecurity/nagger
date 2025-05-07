// Copyright 2025 Nord Security

fn incorrect_std_1() {
    let _instant1 = std::time::Instant::now();
}

fn incorrect_std_2() {
    use std::time::Instant;
    let _instant2 = Instant::now();
}

fn incorrect_tokio_1() {
    let _instant3 = tokio::time::Instant::now();
}

fn incorrect_tokio_2() {
    use tokio::time::Instant;
    let _instant4 = Instant::now();
}


#[allow(instant)]
fn correct_std_1() {    
    let _instant5 = std::time::Instant::now();

    use std::time::Instant;
    let _instant6 = Instant::now();
}

#[allow(instant)]
fn correct_tokio_1() {
    let _instant7= tokio::time::Instant::now();
    
    use tokio::time::Instant;
    let _instant8 = Instant::now();
}

fn main() {}
