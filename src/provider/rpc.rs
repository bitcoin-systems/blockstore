use std::io::{Read, Write};
use std::net::TcpStream;

pub fn request(url: &str) {
  let host = "bsc.drpc.org";
  let port = 80;
  let path = "";

  // Create the POST request body
  let body = "{ \"jsonrpc\":\"2.0\", \"method\":\"eth_getBlockByNumber\", \"params\":[], \"id\":1 }";
  let request = format!(
      "POST {} HTTP/1.1\r\n\
      Host: {}\r\n\
      Content-Type: application/json\r\n\
      Content-Length: {}\r\n\
      Connection: close\r\n\r\n\
      {}",
      path,
      host,
      body.len(),
      body
  );

  // Connect to the server
  let mut stream = TcpStream::connect((host, port)).expect("Failed to connect to the server");

  // Send the request
  stream.write_all(request.as_bytes()).expect("Failed to send request");

  // Read the response
  let mut buffer = Vec::new();
  stream.read_to_end(&mut buffer).expect("Failed to read response");

  // Convert the response from bytes to a string and print it
  let response = String::from_utf8_lossy(&buffer);
  println!("Response: {}", response);
}