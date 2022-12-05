extern crate core;

use std::net::{TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;

fn main() {
    let connection =match TcpStream::connect("127.0.0.1:80") {
        Ok(mut stream) => {
            let msg = b"vier";
            stream.write(msg).unwrap();

            let mut data = [0 as u8; 4];

            match stream.read_exact(&mut data){
                Ok(_)=>{
                    if &data == msg{
                        println!("reply ok")
                    }else{
                        let reply = from_utf8(&data).unwrap();
                        println!("unexpected reply {}", reply)
                    }
                },
                Err(e) => println!("failed to receive data: {}", e)
            }
        }
        Err(e) => {
            panic!("error connecting to host : {}, shutting down", e);
        }
    };
    drop(connection);

}
