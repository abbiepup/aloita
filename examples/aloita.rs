use aloita::startup;
use std::sync::{LazyLock, Mutex};

static INIT: LazyLock<Mutex<Vec<u32>>> = LazyLock::new(|| Mutex::new(Vec::new()));

#[startup(10)]
fn push_1() {
    let mut init = INIT.lock().unwrap();
    init.push(10);
}

#[startup(5)]
fn push_5() {
    let mut init = INIT.lock().unwrap();
    init.push(5);
}

#[startup(0)]
fn push_0() {
    let mut init = INIT.lock().unwrap();
    init.push(0);
}

fn main() {
    dbg!(&INIT);
}
