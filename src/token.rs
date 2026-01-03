#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
   Atom(String),
   Op(char),
   Eof,
}
