mod lexer;
mod token;
use token::Token;
use lexer::Lexer;
use std::{
    fmt,
    io::{self, Write},
};

// Ajuste na estrutura S para usar String
#[derive(Debug, Clone)]
enum S {
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

fn expr_bp(lexer: &mut Lexer, min_bp: u8) -> S {
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

fn compute(s: S) -> Option<i32> {
    match s {
        S::Atom(val) => Some(val.to_string().parse::<i32>().unwrap_or(0)),

        S::Cons(op, children) => {
             // Operador ternário: ? precisa ter acesso aos 3 valores
            if op == '?' && children.len() == 2 {
                // Estrutura esperada: (? condição (: valor_true valor_false))
                let condition = compute(children[0].clone()).unwrap_or(0);
                
                // O segundo filho deve ser uma expressão com ':'
                if let S::Cons(':', colon_children) = &children[1] {
                    if colon_children.len() == 2 {
                        let true_val = compute(colon_children[0].clone()).unwrap_or(0);
                        let false_val = compute(colon_children[1].clone()).unwrap_or(0);

                        return Some(if condition != 0 { true_val } else { false_val });
                    }
                }
            }

            let left = compute(children[0].clone()).unwrap_or(0);
            let mut right = None;
            if children.len() > 1{
                right = compute(children[1].clone());
            }

            match right {
                Some(right) => {match op {
                    '+' => Some(left + right),
                    '-' => Some(left - right),
                    '*' => Some(left * right),
                    '/' => Some(left / right),
                    '=' => { 
                        // Equality
                        if left == right 
                            { Some(1) } 
                        else 
                            { Some(0) }
                    }
                    '<' => {
                        // Adding to variable
                        todo!()
                    }
                    _ => panic!("BAD TOKEN: {}", op),
                }},

                None => {
                    match op {
                    '+' => Some(left),
                    '-' => Some(-1 * left),
                    _ => panic!("BAD TOKEN: {}", op),
                }
                }
            }
        }
    }
}

fn infix_binding_power(op: char) -> Option<(u8, u8)> {
    let res = match op {
        '?'       => (1, 2),
        ':'       => (3, 4),
        '='       => (5, 6),
        '+' | '-' => (7, 8),
        '*' | '/' => (9, 10),
        _ => return None,
    };

    Some(res)
}

fn prefix_binding_power(op: char) -> ((), u8) {
    match op {
        '+' | '-' => ((), 5),
        _ => panic!("BAD TOKEN: {:?}", op),
    }
}

fn postfix_binding_power(op: char) -> Option<(u8, ())> {
    let res = match op {
        '!' | '[' => (7, ()),
        _ => return None
    };

    Some(res)
}

fn expr(input: &str) -> S {
    let mut lexer = Lexer::new(input);
    expr_bp(&mut lexer, 0)
}

fn main() {
    println!("╔════════════════════════════════════════╗");
    println!("║     Calculator Pratt Parser            ║");
    println!("║      'exit' or 'quit' to quit          ║");
    println!("╚════════════════════════════════════════╝");
    println!();

    loop {
        print!(">> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("ERROR TO READ");

        let input = input.trim();

        // Verifica comandos de saída
        if input == "exit" || input == "quit" || input.is_empty() {
            println!("\n👋 BYE!");
            return;
        }

        // Processa a expressão
        match std::panic::catch_unwind(|| {
            let s = expr(input);
            println!("{}", s);
            let eval = compute(s.clone());
            (s, eval)
        }) {
            Ok((s, eval)) => {
                println!("   TREE: {}", s);
                println!("   RESULT: {}\n", eval.unwrap());
            }
            Err(_) => {
                println!("   ❌ ERROR: INVALID EXPRESSION\n");
            }
        }
    }
}
