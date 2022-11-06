use std::io::prelude::*;
use std::net::TcpStream;


pub fn send_message(msg: Vec<u8>, mut address: String, port: u32) -> () {
    
    address.push(':');
    address.push_str(port.to_string().as_str());
    let mut stream = match TcpStream::connect(address) {
        Ok(e) => e,
        _ => {println!("Couldn't establish socket"); return;}
    };
    
    match stream.write(&msg[0..]) {
        Ok(_) => println!("Message sent"),
        _ => println!("Couldn't send message"),
    }
    
}


