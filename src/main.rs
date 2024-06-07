use warp::Filter;
use tokio::sync::mpsc;
use futures::{SinkExt, StreamExt};
use tokio_tungstenite::tungstenite::protocol::Message;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

mod ollama;
mod fabric;

#[derive(Debug, Serialize, Deserialize)]
struct PromptRequest {
    priming: String,
    problem: String,
    solution: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct PromptResponse {
    response: String,
}

async fn handle_connection(ws: warp::ws::WebSocket, prompt_tx: mpsc::Sender<PromptRequest>) {
    let (mut ws_tx, mut ws_rx) = ws.split();
    while let Some(result) = ws_rx.next().await {
        match result {
            Ok(message) => {
                if let Ok(text) = message.to_text() {
                    if let Ok(request) = serde_json::from_str::<PromptRequest>(text) {
                        prompt_tx.send(request).await.unwrap();
                        let response = PromptResponse { response: "Processed prompt response".to_string() };
                        let response_text = serde_json::to_string(&response).unwrap();
                        ws_tx.send(Message::text(response_text)).await.unwrap();
                    }
                }
            }
            Err(e) => {
                eprintln!("WebSocket error: {:?}", e);
                break;
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let (prompt_tx, mut prompt_rx) = mpsc::channel(32);

    let ws_route = warp::path("ws")
        .and(warp::ws())
        .map(move |ws: warp::ws::Ws| {
            let prompt_tx = prompt_tx.clone();
            ws.on_upgrade(move |socket| handle_connection(socket, prompt_tx))
        });

    tokio::spawn(async move {
        warp::serve(ws_route)
            .run(([127, 0, 0, 1], 8080))
            .await;
    });

    let ollama_model = Arc::new(Mutex::new(ollama::OllamaModel::new()));

    while let Some(prompt) = prompt_rx.recv().await {
        let model = ollama_model.lock().unwrap();
        let response = model.process_prompt(&prompt);
        println!("Processed response: {:?}", response);
    }
}
