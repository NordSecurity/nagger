// Copyright 2023 Nord Security
use reqwest::blocking::RequestBuilder;
#[allow(dead_code)]
#[allow(unused_imports)]
#[allow(unused_variables)]
#[allow(unused_must_use)]
use std::time::Duration;
use tokio::{sync::mpsc, time::sleep};

const SPEED: Duration = Duration::from_nanos(100);

async fn mpsc_send(chtx: mpsc::Sender<Vec<u8>>, msg: Vec<u8>) {
    let _ = chtx.send(msg.clone()).await;
}

fn mpsc_sync_send(chtx: std::sync::mpsc::SyncSender<Vec<u8>>, msg: Vec<u8>) {
    let _ = chtx.send(msg.clone());
}

async fn mpsc_send_in_macro(
    mut chrx: mpsc::Receiver<Vec<u8>>,
    chtx: mpsc::Sender<Vec<u8>>,
    msg: Vec<u8>,
) {
    tokio::select! {
        Some(_data) = chrx.recv() => {
            sleep(SPEED);
        },
        _ = sleep(SPEED) => {
            let _ = chtx.send(msg.clone()).await;
        }
    }
}

// should be ignored as this is non-blocking sender
fn mpsc_safe_send(chtx: std::sync::mpsc::Sender<Vec<u8>>, msg: Vec<u8>) {
    let _ = chtx.send(msg.clone());
}

#[cfg(test)]
async fn mpsc_send_in_test(chtx: mpsc::Sender<Vec<u8>>, msg: Vec<u8>) {
    let _ = chtx.send(msg.clone()).await;
}

#[allow(mpsc_blocking_send)]
async fn mpsc_send_allow(chtx: mpsc::Sender<Vec<u8>>, msg: Vec<u8>) {
    let _ = chtx.send(msg.clone()).await;
}

// should be ignored as the sender type is wrong
fn another_send(request: RequestBuilder) {
    let _ = request.send();
}

fn main() {}
