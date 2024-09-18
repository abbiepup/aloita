use aloita::{shutdown, startup};
use libc::printf;

#[startup]
fn before_main_unordered() {
    unsafe { printf(c"Startup Unordered\n".as_ptr()) };
}

#[startup(1)]
fn before_main_ordered() {
    unsafe { printf(c"Startup 1\n".as_ptr()) };
}

#[startup(0)]
fn before_main() {
    unsafe { printf(c"Startup 0\n".as_ptr()) };
}

#[shutdown]
fn after_main() {
    unsafe { printf(c"Shutdown\n".as_ptr()) };
}

fn main() {
    println!("Main");
}
