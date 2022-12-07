extern crate core;

use std::fs::{File, OpenOptions};
use std::net::{TcpStream};
use std::io::{BufRead, BufReader, Read, Write};
use std::str::from_utf8;

fn main() {
    let file = match  File::open("file.txt"){
        Ok(f) => f,
        Err(e) => {
            println!("error, {}",e);
            File::create("file.txt").unwrap()
        }
    };

    let mut buffer = BufReader::new(file);

    let connection =match TcpStream::connect("127.0.0.1:80") {
        Ok(mut stream) => {

            let mut msg = String::new();
            buffer.read_to_string(&mut msg).unwrap();

            stream.write("file.txt\n".as_bytes());
            stream.write(msg.as_ref()).unwrap();


            let mut data = [0 as u8; 2];

            match stream.read_exact(&mut data){
                Ok(_)=>{
                    if &data == "ok".as_bytes(){
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

}
