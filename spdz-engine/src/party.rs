use std::io::Read;

use crate::{field_element::PRIMITIVE_TYPE, network_manager::NetworkManager};
use serde::{Deserialize,Serialize};

#[derive(Serialize,Deserialize,Debug)]
pub struct TripleShare{
    pub a_share: PRIMITIVE_TYPE,
    pub b_share: PRIMITIVE_TYPE,
    pub c_share: PRIMITIVE_TYPE
}

pub struct Party{
    id: u32,
    network: NetworkManager, 
    triples: Vec<TripleShare>
}

impl Party{
    pub fn new(id:u32, network:NetworkManager, file_path: &str)->std::io::Result<Self>{
        let file_handle= std::fs::File::open(file_path)?;
        let triples:Vec<TripleShare> = serde_json::from_reader(file_handle)?;
        Ok(Party { id, network, triples})
    }

    pub fn get_triple(&mut self)->std::io::Result<TripleShare>{
        // can only be used once? otherwise don't pop
        match self.triples.pop(){
            Some(a)=> return Ok(a),
            None => return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Empty list"))
        }
    }
}