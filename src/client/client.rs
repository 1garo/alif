use std::{
    io::prelude::*,
    net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream},
    time::Duration,
    vec, str::{from_utf8, from_utf8_mut},
};

use anyhow::{Context, Result, Ok};

async fn handle_connection(mut stream: TcpStream, ip: Ipv4Addr, port: u16) -> Result<()> {
    let ip = ip.to_string();
    //let mut buf = bufstream::BufStream::new(stream);
    loop {
        let mut buffer = String::new();
        let mut b = vec![0u8; 512];
        // TODO: somehow this does not work unless is a println!
        print!("{ip}:{port}> ");
        let mut stdin = std::io::stdin().lock();
        let mut stdout = std::io::stdout().lock();

        stdout.flush().context("could not flush stdout")?;
        stdin.read_line(&mut buffer).context("should read stdin")?;

        stream.write(buffer.as_bytes()).context("should write on stream")?;
        stream.read(&mut b).context("should read")?;
        stream.flush()?;
        println!("returned: {}", from_utf8(&b)?);
    }
}

#[async_std::main]
async fn main() -> Result<()> {
    let port = 34254;
    let ip = Ipv4Addr::new(127, 0, 0, 1);
    let socket = SocketAddr::new(IpAddr::V4(ip), port);
    let stream = TcpStream::connect_timeout(&socket, Duration::new(1, 0))?;
    handle_connection(stream, ip, port).await?;
    Ok(())
}

