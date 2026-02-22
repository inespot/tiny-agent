use std::io::{self, BufRead, Write};

// https://docs.rs/tokio/latest/tokio/index.html
#[tokio::main]
async fn main() {
    // Load variables from the .env file into the process environment.
    dotenvy::dotenv().ok();

    let api_key = std::env::var("ANTHROPIC_API_KEY")
        .expect("ANTHROPIC_API_KEY must be set in .env or environment");

    let model = std::env::var("MODEL").unwrap_or_else(|_| "claude-sonnet-4-6".into());

    // Reusable HTTP client: https://docs.rs/reqwest/latest/reqwest/
    let client = reqwest::Client::new();

    let stdin = io::stdin();

    println!("\ntiny-agent v0.1.0 (model: {model})");
    println!("Type your prompt and press Enter. Ctrl+D to quit.\n");

    // Main REPL (Read-Eval-Print Loop).
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        let bytes_read = stdin.lock().read_line(&mut input).unwrap();
        if bytes_read == 0 {
            // User pressed Ctrl+D.
            println!("\nBye!");
            break;
        }

        let prompt = input.trim();
        // User just pressed Enter.
        if prompt.is_empty() {
            continue;
        }

        // Call the LLM API and handle the result.
        match ask_model(&client, &api_key, &model, prompt).await {
            Ok(reply) => println!("\n{reply}\n"),
            Err(e) => eprintln!("\nError: {e}\n"),
        }
    }
}


async fn ask_model(
    client: &reqwest::Client,
    api_key: &str,
    model: &str,
    prompt: &str,
) -> Result<String, Box<dyn std::error::Error>> {

    // Build the JSON request body: https://platform.claude.com/docs/en/api/messages.
    let body = serde_json::json!({
        "model": model,
        "max_tokens": 1024,
        "messages": [
            { "role": "user", "content": prompt }
        ]
    });


    let response = client
        .post("https://api.anthropic.com/v1/messages")
        .header("x-api-key", api_key)
        .header("anthropic-version", "2023-06-01")
        .json(&body)
        .send()
        .await?
        .error_for_status()?;

    // Parse the response body as a generic JSON value.
    let json: serde_json::Value = response.json().await?;

    // For a simple text reply, just grab the first "text" field.
    let reply = json["content"][0]["text"]
        .as_str()
        .unwrap_or("[no response]")
        .to_string();

    Ok(reply)
}
