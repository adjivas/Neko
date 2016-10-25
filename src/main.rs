extern crate neko;

use neko::dynamic::Compositer;

fn main() {
 let compositer: Compositer = Compositer::new().unwrap();

    println!("{:?}", compositer );
 // println!("{:?}", compositer.mount_from_git("https://github.com/Arukana/libnya.git", None) );
// println!("{:?}", compositer.uninstall("libnya") );
}
