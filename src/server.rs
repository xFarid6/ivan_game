// Metti tutta la logica del server, come la gestione 
// delle connessioni TCP e il loop principale del server, qui.

// src/server.rs
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{BufReader, BufRead, Write};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};


// Struttura per mantenere lo stato del server
struct ServerState {
    clients: HashMap<usize, TcpStream>, // Mappa degli ID client a stream TCP
    next_client_id: usize,
}

impl ServerState {
    fn new() -> Self {
        ServerState {
            clients: HashMap::new(),
            next_client_id: 0,
        }
    }

    fn add_client(&mut self, stream: TcpStream) -> usize {
        let client_id = self.next_client_id;
        self.clients.insert(client_id, stream);
        self.next_client_id += 1;
        client_id
    }

    fn remove_client(&mut self, client_id: usize) {
        self.clients.remove(&client_id);
    }
}

pub fn start_server(address: &str) {
    let state = Arc::new(Mutex::new(ServerState::new())); // Wrap in Arc and Mutex
    let listener = TcpListener::bind(address).expect("Could not bind to address");
    println!("Server listening on {}", address);
    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let state_clone = Arc::clone(&state); // Clone the Arc for the new thread
                thread::spawn(move || {
                    handle_client(stream, state_clone);
                });
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }
}

fn handle_client(stream: TcpStream, state: Arc<Mutex<ServerState>>) {
    let client_id = {
        let mut state_lock = state.lock().expect("Failed to lock mutex");
        state_lock.add_client(stream.try_clone().expect("Could not clone stream"))
    };
    println!("Client connected: {}", client_id);
    
    let mut reader = BufReader::new(stream.try_clone().expect("Could not clone stream"));
    
    loop {
        let mut buffer = String::new();
        let bytes_read = reader.read_line(&mut buffer).expect("Error reading from stream");
        
        if bytes_read == 0 {
            println!("Client disconnected: {}", client_id);
            {
                let mut state_lock = state.lock().expect("Failed to lock mutex");
                state_lock.remove_client(client_id);
            }
            break;
        }

        println!("Received from {}: {}", client_id, buffer.trim());
        {
            let mut state_lock = state.lock().expect("Failed to lock mutex");
            process_message(&buffer, &mut state_lock);
        }
    }
}

fn process_message(message: &str, state: &mut ServerState) {
    // Logica per gestire diversi tipi di messaggi
    match message.trim() {
        "status" => send_update_to_clients(state, "Server is running\n"),
        _ => println!("Unknown message: {}", message),
    }
}

fn send_update_to_clients(state: &ServerState, message: &str) {
    for (client_id, mut stream) in &state.clients {
        if let Err(e) = stream.write_all(message.as_bytes()) {
            eprintln!("Failed to send message to client {}: {}", client_id, e);
        }
    }
}

fn authenticate(username: &str, password: &str) -> bool {
    // Replace with your actual authentication logic
    username == "player1" && password == "securepassword"
}



// ================== TEST DOWN HERE ==================


#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{TcpListener, TcpStream};
    use std::io::{Write, Read};
    use std::thread;
    use std::time::Duration;

    fn start_test_server(address: &str, state: Arc<Mutex<ServerState>>) {
        let listener = TcpListener::bind(address).expect("Could not bind to address");
        thread::spawn(move || {
            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
                        let state_clone = Arc::clone(&state);
                        thread::spawn(move || {
                            handle_client(stream, state_clone);
                        });
                    }
                    Err(e) => {
                        eprintln!("Error accepting connection: {}", e);
                    }
                }
            }
        });
    }

    #[test]
    fn test_add_client() {
        let mut state = ServerState::new();
        let mock_stream = TcpStream::connect("127.0.0.1:0").unwrap(); // Simulates a client
        let client_id = state.add_client(mock_stream);
        assert_eq!(state.clients.len(), 1);
        assert!(state.clients.contains_key(&client_id));
    }

    #[test]
    fn test_remove_client() {
        let mut state = ServerState::new();
        let mock_stream = TcpStream::connect("127.0.0.1:0").unwrap(); // Simulates a client
        let client_id = state.add_client(mock_stream);
        state.remove_client(client_id);
        assert_eq!(state.clients.len(), 0);
    }

    #[test]
    fn test_send_update_to_clients() {
        let state = Arc::new(Mutex::new(ServerState::new()));
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();

        // Start the test server
        let state_clone = Arc::clone(&state);
        thread::spawn(move || {
            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
                        let state_clone = Arc::clone(&state_clone);
                        thread::spawn(move || {
                            handle_client(stream, state_clone);
                        });
                    }
                    Err(e) => {
                        eprintln!("Error accepting connection: {}", e);
                    }
                }
            }
        });

        let mut client_stream = TcpStream::connect(addr).unwrap();
        state.lock().unwrap().add_client(client_stream.try_clone().unwrap());

        // Simulate sending an update
        let message = "Hello, clients!";
        send_update_to_clients(&state.lock().unwrap(), message);

        // Wait for a bit for the message to be processed
        thread::sleep(Duration::from_millis(100));

        // Verify that the message was sent (mocking the client)
        let mut buffer = String::new();
        let bytes_read = client_stream.read_to_string(&mut buffer).unwrap();
        assert!(bytes_read > 0);
        assert!(buffer.contains(message));
    }

    #[test]
    fn test_handle_client() {
        let state = Arc::new(Mutex::new(ServerState::new()));
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();

        // Start the test server
        start_test_server(&addr.to_string(), Arc::clone(&state));

        // Allow some time for the server to start
        thread::sleep(Duration::from_millis(100));

        // Connect a client
        let mut client_stream = TcpStream::connect(addr).unwrap();
        let message = "status\n";

        // Send a message to the server
        client_stream.write_all(message.as_bytes()).unwrap();

        // Wait a bit for the server to process
        thread::sleep(Duration::from_millis(100));

        // Read response from the server
        let mut response = String::new();
        client_stream.read_to_string(&mut response).unwrap();

        // Check the response from the server
        assert!(response.contains("Server is running")); // Adjust according to your server's expected response
    }
}

