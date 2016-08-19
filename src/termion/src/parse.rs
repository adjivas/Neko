///parse

use std::borrow::Cow;
use event::Key;

//    { Key::Char(' ') => finder.push(bonjour.clone()),
//      _ => bonjour.push(format!("{:?}", i).chars().nth(1).unwrap_or_default()), }; }

///ft_concat
pub fn ft_concat(buf: Key) -> char
{ let mut finder: String = String::from(format!("{:?}", buf));
  for k in finder.chars().enumerate()
  { if k.0 == 6 && k.1 != '\\'
    { return k.1; }}
  'a' }
/*
///ft_concat
pub fn ft_concat(buf: Vec<Key>) -> String
{ let mut finder: Vec<String> = Vec::new();
  let mut bonjour: String = String::new();
  for i in buf
  { finder.push(format!("{:?}", i)); }
  for k in finder
  { for j in k.chars().enumerate()
    { if j.0 == 6 && j.1 != '\\'
      { bonjour.push(j.1); }}}
  bonjour }
*/
///split_spaces
pub fn split_spaces(line: String) -> Vec<String>
{ let bonjour = line.split_whitespace().collect::<Vec<&str>>();
  let mut connard: Vec<String> = Vec::new();
  for i in bonjour
  { let mut coucou: String = String::with_capacity(i.len());
    for j in i.chars()
    { coucou.push(j); }
    connard.push(coucou); }
  connard }

///remove_spaces
pub fn remove_spaces<'a>(input: &'a str)-> Cow<'a, str>
{ if input.contains(' ')
  { let mut buf = String::with_capacity(input.len());
    for c in input.chars()
    { if c != ' '
      { buf.push(c); }}
    return Cow::Owned(buf); }
  return Cow::Borrowed(input); }
