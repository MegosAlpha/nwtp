use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};


fn parse_protocol(parse_in: Vec<u8>, mut stream: &TcpStream) {
    let mut msg = String::from("E MSG_NOT_SET");
    match String::from_utf8(parse_in) {
        Ok(prep) => {
            println!("{:?}", prep);
            let params: Vec<&str> = prep.split(' ').collect();
            let mut set = false;
            if params[0] == "G" {
                if params[1] == "/" {
                    msg = String::from("O Hello! This is the NWTP Ultra-verbose test message. We are on version 0.1.0 as of now and this is the reference implementation.");
                    set = true;
                }
            } else if params[0] == "O" || params[0] == "E" {
                msg = String::from("E THATS_A_SERVER_MESSAGE");
                set = true;
            }
            if !set {
                msg = String::from("E COMMAND_NOT_IMPLEMENTED");
            }
        },
        Err(_) => {
            msg = String::from("E NOT_VALID_UTF8");
        },
    }
    stream.write(msg.as_bytes()).unwrap();
    stream.write(b"\n").unwrap();
}

fn handle_client(mut stream: TcpStream) -> Result<(), std::io::Error> {
    loop {
        let mut n_buf: Vec<u8> = vec![];
        loop {
            let mut init_buf = [0];
            stream.read(&mut init_buf)?;
            if init_buf[0] != b'\n' {
                n_buf.push(init_buf[0]);
            } else {
                break;
            }
        }
        parse_protocol(n_buf, &mut stream);
    }
}

fn main() {
    // accept connections and process them serially
    let listener = TcpListener::bind("127.0.0.1:3736").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                match handle_client(stream) {
                    Ok(_) => println!("Ok!"),
                    Err(e) => println!("{}", e)
                }
            }
            Err(_) => {
                println!("A connection failed!")
            }
        }
    }
}
