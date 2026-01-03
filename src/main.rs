mod infixes;
mod lexer;
mod s_expr;
mod token;
use std::{
    collections::HashMap,
    io::{self, Write},
};
use token::Token;

use crate::s_expr::{compute, expr};

fn main() {
    title();
    let mut vars: HashMap<String, i32> = HashMap::new();
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
        let s = expr(input);
        let eval = compute(s.clone(), &mut vars);        
        println!("   TREE: {}", s);
        println!("   RESULT: {}\n", eval.unwrap_or(0));
        }
    }


fn title() {
    println!("╔════════════════════════════════════════╗");
    println!("║     Calculator Pratt Parser            ║");
    println!("║      'exit' or 'quit' to quit          ║");
    println!("╚════════════════════════════════════════╝");
    println!();
}
