extern crate neko;

use std::io::*;
use std::io;

use neko::dynamic::Compositer;

/// /home/adjivas/test/share/hello/target/debug/libhello.so
fn main() {
    let mut input_line = String::new();
    let mut dy: Compositer = Compositer::default();

    loop {
        print!("> ");
        input_line.clear();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input_line).ok().expect("The read line failed O:");

        input_line.pop();
        println!("{:?}", dy.mount(&input_line, None) );
        dy.start();
    }
}
