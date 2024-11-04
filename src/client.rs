// Metti la logica di connessione del client e il protocollo di 
// comunicazione.

use native_tls::{TlsConnector, TlsStream};
use std::net::{TcpStream, TcpListener};
use std::io::{self, Write, Read};
use std::thread;
use std::sync::{Arc, Mutex};

#[derive(Debug, PartialEq, Eq)]
pub enum ClientState {
    Connected,
    Disconnected,
    Connecting,
}

#[derive(Debug)]
pub struct Client {
    pub stream: Option<TcpStream>, // Make it optional to handle disconnected state
    pub state: ClientState,
    pub last_message: String,
}

impl Client {
    // Create a new client in a disconnected state
    pub fn new() -> Self {
        Client {
            stream: None,
            state: ClientState::Disconnected,
            last_message: String::new(),
        }
    }

    // Set the stream for the client
    pub fn set_stream(&mut self, stream: TcpStream) {
        self.stream = Some(stream);
        self.state = ClientState::Connected;
    }

    // Connect to the server
    pub fn connect(address: &str) -> io::Result<Self> {
        let stream = TcpStream::connect(address)?;
        println!("Connected to the server at {}", address);
        Ok(Client {
            stream: Some(stream),
            state: ClientState::Connected,
            last_message: String::new(),
        })
    }

    pub fn connect_with_tls(address: &str) -> io::Result<TlsStream<TcpStream>> {
        let connector = TlsConnector::new().unwrap();
        let stream = TcpStream::connect(address)?;
        let tls_stream = connector.connect("server_name", stream).expect("TLS connection failed");
        Ok(tls_stream)
    }

    pub fn is_connected(&self) -> bool {
        self.state == ClientState::Connected
    }

    // Disconnect from the server
    pub fn disconnect(&mut self) {
        if let Some(stream) = self.stream.take() {
            // Close the stream explicitly
            let _ = stream.shutdown(std::net::Shutdown::Both);
        }
        self.state = ClientState::Disconnected;
        println!("Disconnected from the server");
    }

    pub fn reconnect(&mut self) -> io::Result<String> {
        Err(io::Error::new(io::ErrorKind::Unsupported, "Unable to reconnect. Function not implemented."))
    }

    // Send a message to the server
    pub fn send_message(&mut self, message: &str) -> io::Result<()> {
        if let Some(ref mut stream) = self.stream {
            stream.write_all(message.as_bytes())?;
            self.last_message = message.to_string(); // Store the last message
            Ok(())
        } else {
            Err(io::Error::new(io::ErrorKind::NotConnected, "Not connected to server"))
        }
    }

    // Receive a message from the server
    pub fn receive_message(&mut self) -> io::Result<String> {
        if let Some(ref mut stream) = self.stream {
            let mut buffer = [0; 512];
            let bytes_read = stream.read(&mut buffer)?;
            let response = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();
            Ok(response)
        } else {
            Err(io::Error::new(io::ErrorKind::NotConnected, "Not connected to server"))
        }
    }

    // Run the client loop
    //  keep the client in a loop as long as it's connected. 
    // You can add additional logic to handle reconnections, timeouts, etc.
    pub fn run(&mut self) -> io::Result<()> {
        while let ClientState::Connected = self.state {
            // Example of handling messages
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            self.send_message(&input.trim())?;

            let response = self.receive_message()?;
            println!("Received from server: {}", response);
        }
        Ok(())
    }

    pub fn authenticate(&self) -> u8 {
        0
    }
}



// ================== TEST DOWN HERE ==================


#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{TcpListener, TcpStream};
    use std::io::{Write, Read};
    use std::thread;
    use std::time::Duration;

    // Mock server that echoes messages back to the client
    fn start_mock_server(address: &str) {
        let listener = TcpListener::bind(address).expect("Could not bind to address");
        thread::spawn(move || {
            for stream in listener.incoming() {
                match stream {
                    Ok(mut stream) => {
                        let mut buffer = [0; 512];
                        while let Ok(bytes_read) = stream.read(&mut buffer) {
                            if bytes_read == 0 { break; } // Connection closed
                            // Echo the message back
                            stream.write_all(&buffer[..bytes_read]).unwrap();
                        }
                    }
                    Err(e) => {
                        eprintln!("Error accepting connection: {}", e);
                    }
                }
            }
        });
    }

    #[test]
    fn test_client_connect_and_echo() {
        let address = "127.0.0.1:8080"; // Use a specific port for the mock server
        start_mock_server(address);

        // Allow some time for the server to start
        thread::sleep(Duration::from_millis(100));

        // Connect the client to the mock server
        let mut client = Client::connect(address).expect("Failed to connect to the server");

        let message = "Hello, server!";
        client.send_message(message).expect("Failed to send message");

        // Receive the echo response from the server
        let response = client.receive_message().expect("Failed to receive message");
        assert_eq!(response, message);
    }

    #[test]
    fn test_send_receive_message() {
        let address = "127.0.0.1:8081"; // Different port for the mock server
        start_mock_server(address);

        // Allow some time for the server to start
        thread::sleep(Duration::from_millis(100));

        let mut client = Client::connect(address).expect("Failed to connect to the server");

        let message = "Ping";
        client.send_message(message).expect("Failed to send message");

        // Wait a moment for the server to process the message
        thread::sleep(Duration::from_millis(100));

        // Receive the echo response from the server
        let response = client.receive_message().expect("Failed to receive message");
        assert_eq!(response, message);
    }

    #[test]
    fn test_client_receive_empty_message() {
        let address = "127.0.0.1:8082"; // Different port for the mock server
        start_mock_server(address);

        // Allow some time for the server to start
        thread::sleep(Duration::from_millis(100));

        let mut client = Client::connect(address).expect("Failed to connect to the server");

        // Try receiving a message without sending anything
        let response = client.receive_message();
        assert!(response.is_err()); // Expect an error since nothing was sent
    }
}
