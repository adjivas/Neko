///nope

use std::io::{stdout, stdin, self};
use std::io::prelude::*;
use termion::event::{Event,Key};
use termion::cursor::{Left,Right,Up,Down,Goto};
use termion::input::{MouseTerminal,TermRead};
use termion::raw::IntoRawMode;
use termion::terminal_size;
use parse::{split_spaces,ft_concat};

trait Bonjour
{ fn curs_pos(&mut self) -> io::Result<Option<String>>;
  fn read_pos<W: Write>(&mut self, writer: &mut W) -> io::Result<Option<String>>
   { let _raw = try!(writer.into_raw_mode());
     self.curs_pos() }}

impl<R: Read> Bonjour for R
{ fn curs_pos(&mut self) -> io::Result<Option<String>>
  { let mut buf = Vec::with_capacity(30);
    for c in self.bytes()
    { match c
      { Err(e) => return Err(e),
        Ok(b'R') => break,
        Ok(c) => buf.push(c), }}
  let string = try!(String::from_utf8(buf).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e)));
  Ok(Some(string)) }}

///structure pour conserver l'état de l'édition de ligne
struct Term
{ curs_x: u16,
  curs_y: u16,
  begin_x: u16,
  begin_y: u16, }

trait TermInfo
{ fn cursor_position(&self) -> io::Result<(u16, u16)>;
  fn go_to_curs(&self); }

impl Term
{ fn new() -> Self
  { Term
    { curs_x: 0,
      curs_y: 0,
      begin_x: 0,
      begin_y: 0, }}}

impl TermInfo for Term
{ fn cursor_position(&self) -> io::Result<(u16, u16)>
  { let stdout = stdout();
    let stdin = stdin();
    let mut stdin = stdin.lock();
    let mut stdout = stdout.lock();
    print!("\x1B[6n\n");
    let connard = stdin.read_pos(&mut stdout);
    let mut x: u16 = 0;
    let mut y: u16 = 0;
    let mut flag = 0;
    if let Ok(Some(connard)) = connard
    { for i in connard.chars()
      { if flag == 0 && i == '['
        { flag = 1; }
        else if flag == 1 && i != ';'
        { y = (y * 10) + (i as u16 - 48); }
        else if flag == 1
        { flag = 2; }
        else if flag == 2 && i != 'R'
        { x = (x * 10) + (i as u16 - 48); }
        else if flag == 2
        { flag = 3; }}
      Ok((x, y)) }
      else
      { Ok((0, 0)) }}
  fn go_to_curs(&self)
  { print!("{}", Goto(self.curs_x, self.curs_y)); }}

struct Neko
{ //image: Vec<Vec<char>>,
  tmp_char: u8,
  coord: (u16, u16),
  size: (u16, u16),
  term: (u16, u16), }

trait NekoInfo
{ fn display(&self);
  fn erase(&self);
  fn switch(&mut self); }

impl Neko
{ fn new(coord: (u16, u16), size: (u16, u16), c: char, term: (u16, u16)) -> Self
  { let mut i = 0;
    while i < size.1 && coord.1 + i < term.1
    { print!("{}", Goto(coord.0, coord.1 + i + 1));
      let mut j = 0;
      while j < size.0 && j + coord.0 <= term.0
      { j += 1;
        print!("{}", c); }
        i += 1; }
    Neko
    { coord: coord,
      size: size,
      tmp_char: c as u8,
      term: term, }}}

impl NekoInfo for Neko
{ fn display(&self)
  { let mut i = 0;
    while i < self.size.1 && self.coord.1 + i < self.term.1
    { print!("{}", Goto(self.coord.0, self.coord.1 + i + 1));
      i += 1;
      let mut j = 0;
      while j < self.size.0
      { j += 1;
        print!("{}", self.tmp_char as char); }}}
  fn switch(&mut self)
  { if self.tmp_char <= 126
    { self.tmp_char += 1; }
    else
    { self.tmp_char = 33; }}
  fn erase(&self)
  { let mut k = 0;
    print!("{}", Goto(self.coord.0, self.coord.1 + 1));
    let mut eraser: String = String::with_capacity(self.size.0 as usize);
    while k < self.size.0
    { k += 1;
      eraser.push(' '); }
    k = 0;
    while k < self.size.1
    { k += 1;
      print!("\r{}{}", eraser, Down(1)); }
    println!("");
    if self.coord.1 + self.size.1 < self.term.1
    { print!("{}", Up(self.size.1 + 1)); }
    else
    { print!("{}", Up(self.size.1)); }}}

fn move_it(way: u8)
{ if way == 0
  { print!("{}", Left(1)); }
  else if way == 1
  { print!("{}", Right(1)); }}

fn move_to(way: i16)
{ if way > 0
  { print!("{}", Right(way as u16)); }
  else if way < 0
  { print!("{}", Left((way * -1) as u16)); }}

///command_line
pub fn command_line() -> Vec<String>
{ let stdout = stdout();
  let mut stdout = stdout.lock();
  let stdin = stdin();
  let ref mut term: Term = Term::new();
  let coord = term.cursor_position().unwrap();
  let size = (8, 5);
  let ref mut neko: Neko = Neko::new(coord, size, '@', terminal_size().unwrap());
  term.curs_x = coord.0 + size.0 + 1;
  term.curs_y = coord.1 + size.1;
  term.begin_x = coord.0 + size.0 + 1;
  term.begin_y = coord.1 + size.1;
  stdout.flush().unwrap();
  term.go_to_curs();
  stdout.flush().unwrap();
  let mut stdout = MouseTerminal::from(stdout.into_raw_mode().unwrap());
  let mut buf: Vec<char> = Vec::new();
  let mut size = 0;
  for c in stdin.events()
  { let b = c.unwrap();
    match b
    { Event::Key(Key::Char('\n')) =>  break,
      Event::Key(Key::Char('\0')) =>  break,
      Event::Key(Key::Char(b)) => { term.curs_x += 1;
        size += 1;
        buf.push(b);
        print!("{}", b) },
      Event::Key(Key::Left) => if term.curs_x > term.begin_x
      { term.curs_x -= 1;
        move_it(0) },
      Event::Key(Key::Right) => if term.curs_x < size + term.begin_x
      { term.curs_x += 1;
        move_it(1) },
      Event::Key(Key::Backspace) => if size > 0 && term.curs_x > term.begin_x
      { size -= 1;
        term.curs_x -= 1;
        move_it(0);
        buf.remove((term.curs_x - term.begin_x) as usize);
        let ref mut u: Vec<_> = buf.drain(((term.curs_x - term.begin_x) as usize)..).collect();
        let mut j = u.clone();
        let taille = u.len();
        for i in u
        { print!("{}", i); }
        buf.append(&mut j);
        print!(" ");
        move_to((taille as i16) * -1);
        move_it(0) },
      Event::Key(Key::Alt(b)) => { term.curs_x += 2;
        size += 1;
        buf.push(b);
        print!("^{}", b) },
      Event::Key(Key::Ctrl(b)) => { term.curs_x += 2;
        size += 1;
        buf.push(b);
        print!("*{}", b) },
      _ => {}, };
    neko.switch();
    neko.display();
    term.go_to_curs();
    stdout.flush().unwrap(); }
  neko.erase();
  split_spaces(ft_concat(buf)) }
