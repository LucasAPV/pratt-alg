mod lexer;
mod token;
mod infixes;
mod s_expr;
use token::Token;
use std::{
    io::{self, Write},
};

use crate::s_expr::{compute, expr};

fn main() {
    title();
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


fn title(){
    println!("╔════════════════════════════════════════╗");
    println!("║     Calculator Pratt Parser            ║");
    println!("║      'exit' or 'quit' to quit          ║");
    println!("╚════════════════════════════════════════╝");
    println!();
}