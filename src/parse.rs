///parse

use std::borrow::Cow;

///ft_concat
pub fn ft_concat(buf: Vec<char>) -> String
{ let mut bonjour: String = String::with_capacity(buf.len());
  for k in buf
  { bonjour.push(k); }
  bonjour }

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
