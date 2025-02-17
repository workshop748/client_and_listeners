use std::thread;
use std::io::{Read,Write};
use std::net::{TcpListener,TcpStream,Shutdown};


fn handling_client(mut stream:TcpStream)
{
    let mut data = [0 as u8;50];
    while match stream.read(&mut data)
    {
Ok(size)=> {
    stream.write(&data[0..size]).unwrap();
    true
},
Err(_)=>
{
println!(" there was an error that occured, terminating connection with{}",stream.peer_addr().unwrap());
stream.shutdown(Shutdown::Both).unwrap();
false
}
    }{}

}
fn main()
{
   //binding to a local host
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
     
    //setting up a itterator to accept connections
    for stream in listener.incoming()
    {
        match stream {
            Ok(stream)=>{
                println!(" new connection {}:",stream.peer_addr().unwrap());
                thread::spawn(move||{
handling_client(stream)
            });
                   
                
            },
        Err(e)=>{
             println!("fail to connect to the client error was {}",e);
        },

        }
    }



}