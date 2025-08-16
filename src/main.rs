use clap::Parser;
use std::fs;

#[derive(Parser)]
struct Cli {
    palavra: String,
}

fn main() {
    let args = Cli::parse();
    
    // TODO: ler arquivo
    let conteudo = fs::read_to_string("dicionario.txt").expect("not possible a read you file");

    for linha in conteudo.lines(){
        let partes: Vec<&str> = linha.split('=').collect();

        if partes[0] == args.palavra {
            println!("{}", partes[1]);
            return;
        }
    }
}