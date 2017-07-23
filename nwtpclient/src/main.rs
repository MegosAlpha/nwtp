use std::net::TcpStream;
use std::io::{Read, Write};
use std::io;

fn main() {
    /*let mut address = String::new();
    println!("Where to?");
    match io::stdin().read_line(&mut address) {
        Ok(_) => {},
        Err(e) => println!("{}", e)
    };*/
    let mut stream = TcpStream::connect("127.0.0.1:3736").unwrap();
    loop {
        let mut address = String::new();
        match io::stdin().read_line(&mut address) {
            Ok(_) => {},
            Err(e) => println!("{}", e)
        }
        stream.write(address.as_bytes()).unwrap();
        let mut n_buf: Vec<u8> = vec![];
        loop {
            let mut msg = [0];
            match stream.read(&mut msg) {
                Ok(_) => {},
                Err(e) => println!("{}", e)
            };
            if msg[0] != b'\n' {
                n_buf.push(msg[0]);
            } else {
                break;
            }
        }
        println!("{}", String::from_utf8(n_buf).unwrap())
    }
}
