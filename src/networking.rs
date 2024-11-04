use bevy::prelude::*;
use bevy::prelude::Timer;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Message {
    command: String,
    data: serde_json::Value, // This allows for flexible data fields
}

// Send JSON
fn send_json_message(stream: &mut TcpStream, message: &Message) -> io::Result<()> {
    let json = serde_json::to_string(message)?;
    stream.write_all(json.as_bytes())?;
    Ok(())
}

// Receive JSON
fn receive_json_message(stream: &mut TcpStream) -> io::Result<Message> {
    let mut buffer = [0; 512];
    let size = stream.read(&mut buffer)?;
    let message: Message = serde_json::from_slice(&buffer[..size])?;
    Ok(message)
}


// This resource has not been initialized yet
pub struct ClientResource {
    pub client: Client,
    pub connection_timer: Timer, // Timer for connection attempts
}

impl ClientResource {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            connection_timer: Timer::from_seconds(10.0, TimerMode::Once), // 10-second timeout
        }
    }
}


fn handle_networking(
    mut client_resource: ResMut<ClientResource>,
    mut commands: Commands,
    time: Res<Time>,
) {
    // Check the state of the client
    match client_resource.client.state {
        ClientState::Connected => {
            // Handle message sending
            let input = get_user_input(); // Your function to get user input
            if let Err(e) = client_resource.client.send_message(&input) {
                eprintln!("Failed to send message: {}", e);
            }

            // Handle receiving messages
            match client_resource.client.receive_message() {
                Ok(response) => {
                    println!("Received: {}", response);
                    // You can add logic here to update game state based on the response
                }
                Err(e) => {
                    eprintln!("Failed to receive message: {}", e);
                }
            }
        }
        ClientState::Disconnected => {
            // Handle reconnection logic
            println!("Client disconnected. Attempting to reconnect...");
            let reconnect_attempt = client_resource.client.reconnect();
            match reconnect_attempt {
                Ok(_) => {
                    println!("Reconnected successfully.");
                    client_resource.client.state = ClientState::Connecting;
                }
                Err(e) => {
                    eprintln!("Reconnection failed: {}", e);
                    // You can add retry logic or cooldowns if needed.
                }
            }
        }
        ClientState::Connecting => {
            // Show loading screen or connection message
            println!("Connecting to the server...");
            client_resource.connection_timer.tick(time.delta());

            if client_resource.connection_timer.finished() {
                eprintln!("Connection attempt timed out.");
                client_resource.client.state = ClientState::Disconnected;
            } else if client_resource.client.is_connected() {
                client_resource.client.state = ClientState::Connected;
                println!("Connected to the server.");
            }
        }
    }
}

// Placeholder function to get user input; replace with actual input handling
fn get_user_input() -> String {
    // Implement your input handling logic here
    "example input".to_string()
}

fn cleanup_client(mut client_resource: ResMut<ClientResource>) {
    client_resource.client.disconnect();
}

