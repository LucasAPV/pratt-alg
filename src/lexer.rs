use crate::Token;


#[derive(Debug)]
pub struct Lexer {
   tokens: Vec<Token>,
}

impl Lexer {
   pub fn new(input: &str) -> Lexer {
      let mut tokens = Vec::new();
      let mut chars = input.chars().peekable();
      
      while let Some(&c) = chars.peek() {
         if c.is_ascii_whitespace() {
               chars.next();
               continue;
         }
         
         // Se é dígito ou letra, agrupa todos consecutivos
         if c.is_ascii_alphanumeric() {
               let mut atom = String::new();
               while let Some(&ch) = chars.peek() {
                  if ch.is_ascii_alphanumeric() {
                     atom.push(ch);
                     chars.next();
                  } else {
                     break;
                  }
               }
               tokens.push(Token::Atom(atom));
         } 
         // Caso contrário, é um operador
         else {
               tokens.push(Token::Op(c));
               chars.next();
         }
      }
      
      tokens.reverse();
      Lexer { tokens }
   }

   pub fn next(&mut self) -> Token {
      self.tokens.pop().unwrap_or(Token::Eof)
   }
   
   pub fn peek(&mut self) -> Token {
      self.tokens.last().cloned().unwrap_or(Token::Eof)
   }
}