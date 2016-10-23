extern crate neko;

use neko::graphic::Manager;

fn main() {
  let mut manager: Manager = Manager::default();
  
  manager.insert_from_spritefile("/home/ubuntu/workspace/Neko_graphic/.neko/config/sprite/1.cns");
}