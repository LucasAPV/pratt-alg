use std::{fmt};
use std::collections::HashMap;

use crate::{infixes::{infix_binding_power, postfix_binding_power, prefix_binding_power}, lexer::Lexer, token::Token};

#[derive(Debug, Clone)]
pub enum S {
   Atom(String),
   Cons(char, Vec<S>),
}

impl fmt::Display for S {
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      match self {
         S::Atom(i) => write!(f, "{}", i),
         S::Cons(head, rest) => {
               write!(f, "({}", head)?;
               for s in rest {
                  write!(f, " {}", s)?
               }
               write!(f, ")")
         }
      }
   }
}

pub fn expr_bp(lexer: &mut Lexer, min_bp: u8) -> S {
   let mut lhs = match lexer.next() {
      Token::Atom(it) => S::Atom(it),
      
      Token::Op('(') => {
         let lhs = expr_bp(lexer, 0);
         assert_eq!(lexer.next(), Token::Op(')'));
         lhs
      }

      Token::Op(op) => {
         let ((), r_bp) = prefix_binding_power(op);
         let rhs = expr_bp(lexer, r_bp);
         S::Cons(op, vec![rhs])
      }

      t => panic!("bad token: {:?}", t),
   };

   loop {
      let op = match lexer.peek() {
         Token::Eof => break,
         Token::Op(op) => op,
         t => panic!("bad token: {:?}", t),
      };

      if let Some((l_bp, ())) = postfix_binding_power(op) {
         if l_bp < min_bp {
               break;
         }
         lexer.next();

         lhs = if op == '[' {
               let rhs = expr_bp(lexer, 0);
               assert_eq!(lexer.next(), Token::Op(']'));
               S::Cons(op, vec![lhs, rhs])
         } else {
               S::Cons(op, vec![lhs])
         };

         continue;
      }

      if let Some((l_bp, r_bp)) = infix_binding_power(op){
         if l_bp < min_bp {
               break;
         }

         lexer.next();
         let rhs = expr_bp(lexer, r_bp);

         lhs = S::Cons(op, vec![lhs, rhs]);
         continue;
      }

      break;
   }
   lhs
}

pub fn compute(s: S, vars: &mut HashMap<String, i32>) -> Option<i32> {
   match s {
      S::Atom(val) => {
         if let Ok(num) = val.to_string().parse::<i32>(){
            Some(num)
         } else {
            vars.get(&val.to_string()).copied()
         }
      },

      S::Cons(op, children) => {
            if op == '<' && children.len() >= 2 {
               if let S::Atom(var_name) = &children[0]{
                  let value = compute(children[1].clone(), vars).unwrap_or(0);
                  vars.insert(var_name.to_string(), value);
                  return Some(value)
               }
            }
            
            
            // Operador ternário: ? precisa ter acesso aos 3 valores
         if op == '?' && children.len() == 2 {
               // Estrutura esperada: (? condição (: valor_true valor_false))
               let condition = compute(children[0].clone(), vars).unwrap_or(0);
               
               // O segundo filho deve ser uma expressão com ':'
               if let S::Cons(':', colon_children) = &children[1] {
                  if colon_children.len() == 2 {
                     let true_val = compute(colon_children[0].clone(),vars).unwrap_or(0);
                     let false_val = compute(colon_children[1].clone(), vars).unwrap_or(0);

                     return Some(if condition != 0 { true_val } else { false_val });
                  }
               }
         }

         let left = compute(children[0].clone(), vars).unwrap_or(0);
         let mut right = None;
         if children.len() > 1{
               right = compute(children[1].clone(), vars);
         }

         match right {
               Some(right) => {match op {
                  '+' => Some(left + right),
                  '-' => Some(left - right),
                  '*' => Some(left * right),
                  '/' => Some(left / right),
                  '^' => Some(left.pow(right as u32)),
                  '=' => { 
                     // Equality
                     if left == right 
                           { Some(1) } 
                     else 
                           { Some(0) }
                  },
                  '>' => {
                  if left > right 
                        { Some(1) } 
                  else 
                        { Some(0) }
                  }
                  _ => panic!("BAD TOKEN: {}", op),
               }},

               None => {
                  match op {
                  '+' => Some(left),
                  '-' => Some(-1 * left),
                  '@' => Some((left as f64).sqrt() as i32),

                  _ => panic!("BAD TOKEN: {}", op),
               }
               }
         }
      }
   }
}

pub fn expr(input: &str) -> S {
   let mut lexer = Lexer::new(input);
   expr_bp(&mut lexer, 0)
}