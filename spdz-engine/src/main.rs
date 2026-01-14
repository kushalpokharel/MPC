
mod field_element;
mod party;
mod triple_generator;
mod network_manager;

use field_element::FieldElement;
use triple_generator::TripleGenerator;
use party::Party;
use network_manager::NetworkManager;
use std::io;
use std::env;

fn main() -> io::Result<()>{

    // parse the enviroment variable, check the id (id=1 make server, call listen)
    // id=anythingelse make client, call connect
    let arguments:Vec<String> = env::args().collect();
    let id = arguments[1].parse::<u32>().expect("Couldn't unwrap the string to u32");
    let mut network = if id==1{
        NetworkManager::listen()?
    }else{
        NetworkManager::connect("0.0.0.0:80")?
    };

    if id==1{
        // from server
        network.send_data("Hello".as_bytes())?;
    }
    if id==2{
        let mut buffer = [0u8;20];
        network.receive_data(&mut buffer)?;
        println!("Client received data {:?} in buffer ", str::from_utf8(&buffer));

        let mut party = Party::new(id, network, "./data/triple_2.json")?;
        let triple = party.get_triple()?;
        println!("The triple is {:?} ", triple);
    }
    



    Ok(())
}

