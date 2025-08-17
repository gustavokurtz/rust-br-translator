use clap::Parser;
use reqwest::Client; 
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct TranslateRequest {
    q: String,
    source: String,
    target: String,
    format: String,
    alternatives: u32,
    api_key: String,
}

#[derive(Deserialize, Debug)]
struct TranslateResponse {
    #[serde(rename = "translatedText")]
    translated_text: String,
    alternatives: Vec<String>,
}


#[derive(Parser)]
struct Cli {
    palavra: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    let args = Cli::parse();

    let request_body = TranslateRequest {
        q: args.palavra, // Texto a ser traduzido
        source: "pt-BR".to_string(),
        target: "en".to_string(),
        format: "text".to_string(),
        alternatives: 3,
        api_key: "".to_string(), 
    };

    let response = client
        .post("https://gustavodev.tech/translate")
        .header("Content-Type", "application/json")
        .json(&request_body) // Automaticamente serializa para JSON
        .send()
        .await?;

    if response.status().is_success() {
        let result: TranslateResponse = response.json().await?;
        println!("Tradução: {}", result.translated_text);
        println!("Alternativas: {:?}", result.alternatives);
    } else {
        let status = response.status(); // Salva o status antes de consumir response
        let error_text = response.text().await?;
        println!("Erro: {} - {}", status, error_text);
    }

    Ok(())
}