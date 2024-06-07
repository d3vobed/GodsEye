use warp::Filter;
use tokio::sync::mpsc;
use futures::{SinkExt, StreamExt};
use tokio_tungstenite::tungstenite::protocol::Message;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

// Define the structure for handling prompts
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

// Function to handle WebSocket connections
async fn handle_connection(ws: warp::ws::WebSocket, prompt_tx: mpsc::Sender<PromptRequest>) {
    let (mut ws_tx, mut ws_rx) = ws.split();
    while let Some(result) = ws_rx.next().await {
        match result {
            Ok(message) => {
                if let Ok(text) = message.to_text() {
                    if let Ok(request) = serde_json::from_str::<PromptRequest>(text) {
                        // Send prompt to processing
                        prompt_tx.send(request).await.unwrap();
                        // Receive processed prompt (dummy response for example)
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

    // WebSocket route
    let ws_route = warp::path("ws")
        .and(warp::ws())
        .map(move |ws: warp::ws::Ws| {
            let prompt_tx = prompt_tx.clone();
            ws.on_upgrade(move |socket| handle_connection(socket, prompt_tx))
        });

    // Start WebSocket server
    tokio::spawn(async move {
        warp::serve(ws_route)
            .run(([127, 0, 0, 1], 8080))
            .await;
    });

    // Handle prompts (dummy implementation)
    while let Some(prompt) = prompt_rx.recv().await {
        println!("Received prompt: {:?}", prompt);
        // Process prompt using Ollama model here
    }
}
