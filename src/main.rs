
extern crate ms;
use ms::get_line::command_line;

fn main()
{ loop
  { let tmp = command_line(); 
    for i in tmp
    { if i == "exit"
      { return }}
    print!("\r"); }}
