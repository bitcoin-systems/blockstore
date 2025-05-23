use std::io::{Read, Write};
use std::net::TcpStream;

pub fn rpc () {
  // Replace this with your Ethereum node's IP and port
  let host = "public-node.rsk.co";
  let port = 80;
  let address = format!("{host}:{port}");

  let mut stream = TcpStream::connect(address).unwrap();

  // JSON-RPC request payload
  let json_rpc = r#"{
      "jsonrpc":"2.0",
      "method":"eth_getBlockByNumber",
      "params":["latest", false],
      "id":1
  }"#;

  let request = format!(
      "POST / HTTP/1.1\r\n\
       Host: {host}:{port}\r\n\
       Content-Type: application/json\r\n\
       Content-Length: {}\r\n\
       Connection: close\r\n\
       \r\n\
       {}",
      json_rpc.len(),
      json_rpc
  );

  let _ = stream.write_all(request.as_bytes());

  let mut response = String::new();
  let _ = stream.read_to_string(&mut response);

  println!("Response:\n{}", response);
}