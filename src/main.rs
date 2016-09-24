extern crate neko;

use neko::dynamic::Compositer;

fn main() {
  let mut compositer: Compositer = Compositer::default();

  println!("{:?}", compositer.mount_from_git("https://github.com/Arukana/libnya.git", None));
}
