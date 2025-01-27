use std::net;
use std::io;
use std::net::TcpListener;

fn main()
{
   //binding to a local host
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
     
    //setting up a itterator to accept connections
    for stream in listener.incoming()
    {
        match stream {
            Ok(stream)=>{
                
            },
Err(e)=>{
    println!("fail to connect to the client error was {}",e);
},

        }
    }



}