use async_std::net::TcpListener;
use async_std::net::TcpStream;
use async_std::prelude::*;
use async_std::task::spawn;

// Request trigger with: curl http://localhost:8080 --http2-prior-knowledge --verbose

async fn handle_stream(mut input: TcpStream) {

  // Preflight
  let mut pre_bytes = vec![0u8; 24];
  input.read_exact(&mut pre_bytes).await;
  println!("Preflight: {:?}", String::from_utf8(pre_bytes).unwrap());

  loop {

    let mut length_bytes = vec![0u8; 3];
    input.read_exact(&mut length_bytes).await;
    let length = u32::from_be_bytes([0u8, length_bytes[0], length_bytes[1], length_bytes[2]]);

    println!("Length: {:?}", length);

    // parse frame type
    let mut type_bytes = vec![0u8];
    input.read_exact(&mut type_bytes).await;
    let frame_type: u8 = type_bytes[0];

    println!("Frame type: {:?}", frame_type);

    if frame_type == 4 { // settings frame
      // parse frame flags
      let mut flags_bytes = vec![0u8];
      input.read_exact(&mut flags_bytes).await;
      let flags: u8 = flags_bytes[0];

      println!("Flags: {:?}", flags);

      // parse frame stream id
      let mut stream_bytes = vec![0u8; 4];
      input.read_exact(&mut stream_bytes).await;
      let stream = u32::from_be_bytes([stream_bytes[0], stream_bytes[1], stream_bytes[2], stream_bytes[3]]);

      println!("Stream: {:?}", stream);

      // parse frame payload
      let mut i = 0;
      while i < length {
          let mut key_bytes = vec![0u8; 2];
          input.read_exact(&mut key_bytes).await;

          let mut value_bytes = vec![0u8; 4];
          input.read_exact(&mut value_bytes).await;

          let key = u16::from_be_bytes([key_bytes[0], key_bytes[1]]);
          let value = u32::from_be_bytes([value_bytes[0], value_bytes[1], value_bytes[2], value_bytes[3]]);

          println!("Key: {:?}", key);
          println!("Value: {:?}", value);

          i+=6;
      }

      println!("Settings frame read - responding with empty settings frame");
      let mut settings_bytes: Vec<u8> = Vec::new();
      // Length 0 - 3 bytes
      settings_bytes.push(0); // 00
      settings_bytes.push(0); // 00
      settings_bytes.push(0); // 00
      // Type 4 - 1 byte
      settings_bytes.push(4); // 04
      // flags none - 1 bytes
      settings_bytes.push(0); // 00
      // stream id 0 - 4 bytes
      settings_bytes.push(0); // 00
      settings_bytes.push(0); // 00
      settings_bytes.push(0); // 00
      settings_bytes.push(0); // 00

      input.write(settings_bytes.as_slice()).await.unwrap();
      input.flush().await.unwrap();
    }
  }
}

async fn main() -> std::io::Result<()> {
  let listener = TcpListener::bind("127.0.0.1:8080").await?;
  let mut incoming = listener.incoming();
  while let Some(stream) = incoming.next().await {
      let stream = stream.unwrap();
      spawn(handle_stream(stream));
  }
  Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
  
    #[async_std::test]
    async fn run_server() {
        main().await.unwrap();
    }
}