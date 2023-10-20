use std::io::Result;
use std::net::TcpListener;
use std::thread;

mod sub;

fn main() -> Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8080")?;
    while let Ok((stream, _)) = listener.accept() {
        thread::spawn(|| sub::tcp_handler(stream));
    }
    Ok(())
}

