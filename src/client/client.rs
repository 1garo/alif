use std::{
    io::prelude::*,
    net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream},
    str::from_utf8,
    time::Duration,
    vec,
};

fn main() -> std::io::Result<()> {
    let port = 34254;
    let ip = Ipv4Addr::new(127, 0, 0, 1);
    let socket = SocketAddr::new(IpAddr::V4(ip), port);
    let stream = TcpStream::connect_timeout(&socket, Duration::new(1, 0));
    match stream {
        Ok(mut stream) => {
            let ip = ip.to_string();
            loop {
                let mut buffer = String::new();
                print!("{ip}:{port}> ");
                let stdin = std::io::stdin();
                let mut stdout = std::io::stdout();
                let mut handle = stdin.lock();
                stdout.flush()?;

                handle.read_line(&mut buffer)?;
                stream.write(buffer.as_bytes())?;
            }
        }
        Err(error) => panic!("Problem opening the file: {:?}", error),
    }
}
