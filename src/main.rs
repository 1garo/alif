use std::io::{Read, Write};
use std::str::from_utf8;
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    // ...
    loop {
        let mut buf = vec![0u8; 512];
        let bytes_read = stream.read(&mut buf).expect("should read stream");
        if bytes_read == 0 {
            break
        }
        stream.write("its ok: ".as_bytes()).unwrap();
        stream.flush().unwrap();
        print!("return: {}", from_utf8(&buf).unwrap());
    }
}

fn main() -> std::io::Result<()> {
    let port = "34254";
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port))?;
    println!("listening on port: localhost:{port}");

    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}
