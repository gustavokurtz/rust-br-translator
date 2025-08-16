use clap::Parser;
use std::collections::HashMap;

#[derive(Parser)]
struct Cli {
    palavra: String,
}

fn main() {
    let args = Cli::parse();
    
    let dicionario = HashMap::from([
        ("amarelo", "yellow"),
        ("casa", "house"),
        ("carro", "car"),
        ("água", "water"),
        ("livro", "book"),
    ]);

    if let Some(traducao) = dicionario.get(&*args.palavra){
        println!("{}", traducao)
    } else {
        println!("Palavra não encontrada");
    }
   
}