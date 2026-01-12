use crate::field_element::FieldElement;

pub struct Party{
    // the secret_share of the alpha for this party (alpha is our key for the MAC)
    alpha_share: FieldElement,
    // this is the local secret to this party to check for the alpha shares if they are correctly 
    // given by all the parties
    beta: FieldElement,
    // this is the shares for the alpha_beta (MAC for the beta) for all of the parties i.e 
    // alpha_beta_share[i] of this party would contribute to the alpha_beta of the party i. 
    // required for the verification of alpha.
    alpha_beta_share: Vec<FieldElement>
}

impl Party{
    pub fn new(alpha_share:FieldElement, beta:FieldElement, alpha_beta_share:Vec<FieldElement>)->Self{
        Party{
            alpha_share,
            beta,
            alpha_beta_share
        }
    }
    pub fn change_share_of_party_i(&mut self, new_share:FieldElement, party_idx:usize){
        self.alpha_beta_share[party_idx-1] = new_share;
    }
    pub fn change_alpha(&mut self, new_alpha:FieldElement){
        self.alpha_share = new_alpha;
    }

    pub fn broadcast_alpha_share(&self)->&FieldElement{
        &self.alpha_share
    }
    pub fn broadcast_beta_share(&self)->&FieldElement{
        &self.beta
    }
    pub fn broadcast_alpha_beta_share(&self)->&Vec<FieldElement>{
        &self.alpha_beta_share
    }
    pub fn check_authenticity_of_reconstructed_alpha(&self, alpha_shares:&Vec<FieldElement>, alpha_beta_shares:&Vec<FieldElement>)-> bool{
        let alpha:FieldElement = alpha_shares.into_iter().fold(FieldElement::new(0), |acc, new|*new+acc);
        let alpha_beta:FieldElement = alpha_beta_shares.iter().sum();
        println!("Alpha {:?} beta{:?} alpha beta {:?}", alpha, self.beta, alpha_beta);
        if alpha*self.beta==alpha_beta{
            true
        }
        else{
            false
        }
    }
}