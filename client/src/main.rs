use std::net::TcpStream;
use std::io::{self, Read, Write};
use std::str::from_utf8;
fn main()
{
    match TcpStream::connect("127.0.0.1:8080")
    {
        Ok(mut stream)=>{
        println!("succesfully connected  to server in port 7877");
        let mut mssg= String::new();
        println!("Please enter some text to send to a server");
        io::stdin().read_line(&mut mssg).expect("Failed to read line");

        let byte_message= mssg.as_bytes();
        stream.write(byte_message).unwrap();
        println!("sent message");
        let mut data = [0 as u8;8];

        match stream.read_exact(&mut data)
        {
            Ok(_)=> {
                let response= from_utf8(&data).unwrap();
                println!("{}",response);
            }
            Err(e)=>
            {
                println!("Failed to receive data: {}",e);
            }
        }
        }
        Err(e)=>{
            println!("Failed to connect : {}",e);
        }
    }
println!("Terminated.");

}