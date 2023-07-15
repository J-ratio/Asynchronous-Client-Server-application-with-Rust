use std::{net::TcpStream, io::Read};

fn main() -> std::io::Result<()>{
    let mut stream = TcpStream::connect(("www.google.com", 8080))?;

    let mut r = String::from("");

    stream.read_to_string(&mut r)?;

    println!("{}",r);

    Ok(())
}
