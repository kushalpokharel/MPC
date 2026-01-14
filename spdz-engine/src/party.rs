use std::io::{self, Read};

use crate::{field_element::{FieldElement, PRIMITIVE_TYPE}, network_manager::NetworkManager};
use serde::{Deserialize,Serialize};
use serde_json::Serializer;

#[derive(Serialize,Deserialize,Debug)]
pub struct TripleShare{
    pub a_share: FieldElement,
    pub b_share: FieldElement,
    pub c_share: FieldElement
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
        println!("Out triples {:?}", triples);
        Ok(Party {id, network, triples})
    }

    pub fn get_triple(&mut self)->std::io::Result<TripleShare>{
        // can only be used once? otherwise don't pop
        match self.triples.pop(){
            Some(a)=> return Ok(a),
            None => return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Empty list"))
        }
    }

    pub fn send_share(&mut self, my_share:FieldElement)->io::Result<()>{
        let share = serde_json::to_string(&my_share)?;
        self.network.send_data(share.as_bytes())
    }

    pub fn receive_share(&mut self)->io::Result<FieldElement>{
        let mut val = [0u8;20];
        self.network.receive_data(&mut val)?;
        let strval= str::from_utf8(&val).expect("Not parseable");
        println!("Recieved data {strval}");
        // FIX: Trim null bytes explicitly
        let clean_str = strval.trim_matches(char::from(0));
        let parsed:i32 = clean_str.parse().expect("not parseable");
        let fe:FieldElement = FieldElement::new(parsed);
        return Ok(fe);
    }

    /// 1. Broadcast my share of the value.
    /// 2. Receive the other party's share.
    /// 3. Return the reconstructed secret (shifted x1 + shifted x2).
    pub fn open(&mut self, my_share: FieldElement) -> io::Result<FieldElement> {
        // Send my share
        self.send_share(my_share)?;
        
        // Receive their share
        let their_share = self.receive_share()?;
        println!("party {:?} shares myshare {:?} their_share {:?}", self.id, my_share, their_share);
        // Reconstruct (Additive Secret Sharing)
        Ok(my_share+their_share)
    }

    /// The Protocol:
    /// 1. Pop a triple ([a], [b], [c]) from the stack.
    /// 2. Calculate [d] = [x] - [a] and [e] = [y] - [b].
    /// 3. OPEN d and e (over the network).
    /// 4. Calculate [z] formula.
    pub fn beaver_multiply(&mut self, x: FieldElement, y: FieldElement) -> io::Result<FieldElement> {
        // 1. Get Triple
        // (In a real app, panic if empty)
        let triple = self.triples.pop().expect("Empty triple vector");
        
        // 2. Local subtraction
        let d_share = x - triple.a_share;
        let e_share = y - triple.b_share;
        
        // 3. Network Round Trip (The expensive part)
        let d = self.open(d_share)?;
        let e = self.open(e_share)?;
        
        // 4. Local Reconstruction of [z]
        // Formula: [z] = [c] + d*[b] + e*[a] + d*e (only added to one party's share)
        
        let mut z_share = triple.c_share+ d*triple.b_share+ e*triple.a_share;
        // The constant term (d * e) must only be added ONCE total.
        // Convention: Party 1 adds it. Party 2 adds nothing.
        println!("{:?} {:?}, {:?}", d,e, d*e);
        println!("{:?} {:?}, {:?}", triple.c_share, triple.b_share, triple.a_share);

        if self.id == 1 {
            z_share = z_share+(d*e);
        }
        
        Ok(z_share)
    }
}