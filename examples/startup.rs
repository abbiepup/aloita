use libc::printf;
use aloita::startup;

#[startup]
fn before_main() {
    unsafe { printf(c"Startup\n".as_ptr()) };
}

fn main() {
    println!("Main");
}
