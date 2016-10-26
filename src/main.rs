extern crate neko;

use neko::dynamic::Compositer;
use std::path::PathBuf;

fn main() {
    let mut compositer: Compositer = Compositer::new().unwrap();

    println!("{:?}", compositer.build(
        &PathBuf::from("/home/adjivas/.neko/git/arukana@libnya"),
        "arukana@libnya",
        )
    );
}
