
mod field_element;
mod party;

use field_element::FieldElement;
use std::iter;
use party::Party;

fn main() {
    // 1. Setup the global alpha and beta for each party(two party only for illustration)
    let modulus = 7919;
    let alpha = FieldElement::new(50,modulus);
    // for each party one beta
    let betas:Vec<FieldElement> = vec![FieldElement::new(12,modulus), FieldElement::new(15,modulus)];
    let alpha_1 = FieldElement::new(15, modulus);
    let alpha_2:FieldElement = &alpha-&alpha_1;
    let alphas = vec![alpha_1, alpha_2];
    
    // 2. Setup the alpha_beta for each and secret share between the parties (manually)
    // ideally, would like to call a function secret_share that would generate n-1 random shares
    // and n_th share such that it would hold summation with the secret
    let alpha_beta:Vec<FieldElement> = betas.iter().map(|beta| &alpha*beta).collect();
    let alpha_beta0_1 = FieldElement::new(123, modulus);
    let alpha_beta0_2 = &alpha_beta[0]-&alpha_beta0_1;
    let alpha_beta1_1 = FieldElement::new(527, modulus);
    let alpha_beta1_2 = &alpha_beta[1]-&alpha_beta1_1;

    // 3. Setup party with all the available information from the offline source (main function)
    // these should also have many other setup vairables like multiple beaver tripes, along with 
    // secret shared random variables but we are dedicated to proving the alpha correctness for now.
    let party1 = Party::new(alphas[0], betas[0], vec![alpha_beta0_1, alpha_beta1_1]);
    let mut party2 = Party::new(alphas[1], betas[1], vec![alpha_beta0_2, alpha_beta1_2]);
    
    // 4. Some calculation happens here (cicuit evaluation)

    // 5. Reveal the alpha at the end and check if the provided shares from the parties rightly 
    //  construct our \alpha*\beta using our secret \beta and recovering alpha_beta for that party.
    let party_1_alpha = party1.broadcast_alpha_share().clone();
    let mut party_2_alpha = party2.broadcast_alpha_share().clone();
    let party_1_alpha_beta = party1.broadcast_alpha_beta_share();
    let party_2_alpha_beta = party2.broadcast_alpha_beta_share();
    let party_1_beta = party1.broadcast_beta_share();
    let party_2_beta = party2.broadcast_beta_share();
    println!("{:?} {:?}", party_1_alpha_beta, party_2_alpha_beta);
    // index zero alpha beta shares equals to party0's alpha beta shares
    let party1_alpha_beta = vec![party_1_alpha_beta[0], party_2_alpha_beta[0]];
    let party2_alpha_beta = vec![party_1_alpha_beta[1], party_2_alpha_beta[1]];
    let status = party1.check_authenticity_of_reconstructed_alpha(&vec![party_1_alpha, party_2_alpha], &party1_alpha_beta);
    assert!(status == true);
    // Simulating the change of alpha from party 2.
    party2.change_alpha(FieldElement::new(27, modulus));
    // Failing case 
    party_2_alpha = party2.broadcast_alpha_share().clone();
    let failing_status = party1.check_authenticity_of_reconstructed_alpha(&vec![party_1_alpha, party_2_alpha], &party1_alpha_beta);
    assert!(failing_status == false);

}

