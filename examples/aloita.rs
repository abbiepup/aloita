use aloita::{shutdown, startup};
use std::sync::Mutex;

static INIT: Mutex<Vec<u32>> = Mutex::new(Vec::new());

#[startup(10)]
fn push_10() {
    let mut init = INIT.lock().unwrap();
    init.push(010);
}

#[startup(5)]
fn push_5() {
    let mut init = INIT.lock().unwrap();
    init.push(5);
}

#[startup]
fn push_5000() {
    let mut init = INIT.lock().unwrap();
    init.push(5000);
}

#[startup(0)]
fn push_0() {
    let mut init = INIT.lock().unwrap();
    init.push(0);
}

fn main() {
    dbg!(&INIT);
}
