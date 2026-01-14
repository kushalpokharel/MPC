use std::{fmt::Error, io::{Read, Write}, net::{SocketAddr, TcpListener, TcpStream}, thread, time::Duration};
use std::io::{self};
pub struct NetworkManager{
    stream:TcpStream
}

impl NetworkManager{
    pub fn listen()-> io::Result<Self>{
        // whichever party calls this acts a server to listen for connections from the clients
        // after the clients connect to the server, everyone is client/peer
        let socket = TcpListener::bind("0.0.0.0:80")?; //hardcode port for now
        let connection= socket.accept()?;
        let stream = connection.0;
        println!("Accepted a stream from {:?}",  connection.1);
        Ok(NetworkManager{stream})
    }

    pub fn connect(address:&str)-> io::Result<Self>{
        // client calls this with the address to the server it wants to connect.
        let mut retries = 0;
        loop{
            let connection = TcpStream::connect(address);
            match connection{
                Ok(stream)=> return Ok(NetworkManager{stream}),
                Err(e) =>{
                    // retry here after 1 second
                    thread::sleep(Duration::from_secs(1));
                    if retries>10{
                        return Err(e);
                    }
                    retries+=1;
                }
            };
        }
        
    }

    pub fn send_data(&mut self, data:&[u8])->io::Result<()>{
        let written_bytes = self.stream.write(data)?;
        if written_bytes == data.len(){
            Ok(())
        }
        else{
            Err(io::Error::new(io::ErrorKind::Interrupted, "Transfer couldn't be completed"))
        }
    }
    pub fn receive_data(&mut self, data:&mut [u8])->io::Result<()>{
        self.stream.read(data)?;
        Ok(())
        
    }
}