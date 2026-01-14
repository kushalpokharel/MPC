// Triple generator simulation
    println!("--- SPDZ Offline Phase Simulation ---");

    let dealer = TripleGenerator::new();
    let (p1_triple, p2_triple) = dealer.generate_triple();

    println!("Generated Triples!");
    println!("Party 1 has shares of a: {:?}", p1_triple.a);
    println!("Party 2 has shares of a: {:?}", p2_triple.a);

    // Verification (Just to prove the math works)
    let recon_a = p1_triple.a + p2_triple.a;
    let recon_b = p1_triple.b + p2_triple.b;
    let recon_c = p1_triple.c + p2_triple.c;

    println!("Reconstructed a: {:?}", recon_a); // Should be 5
    println!("Reconstructed b: {:?}", recon_b); // Should be 7
    println!("Reconstructed c: {:?}", recon_c); // Should be 35
    
    // Check property: a * b = c ?
    let product = recon_a * recon_b;
    println!("Does a * b == c? {}", product == recon_c);


    // simulate the x*y=z calculation for two parties holding the shares of x and y.
    let x = FieldElement::new(22);
    let y = FieldElement::new(11); 
    let x1 = FieldElement::new(9); // party one x
    let x2 = x - x1; // party two x
    let y1 = FieldElement::new(3); // party one y
    let y2 = y - y1; // party two y

    // start of the protocol.
    // parties share the shifted value of their shares i.e x1-a
    let shifted_party1_x = x1 - p1_triple.a;
    let shifted_party2_x = x2 - p2_triple.a;
    let shifted_party1_y = y1 - p1_triple.b;
    let shifted_party2_y = y2 - p2_triple.b;

    // now shares of z can be derived with 
    // z = x*y = (d+a)*(e+b) = d*e+a*e+ab+b*d => 
    // share of a*b=c => we need d and e by opening their shares
    // simulate d1 as broadcasted by part1.

    // Since a and b are random, this doesn't leak any information
    let d = shifted_party1_x+shifted_party2_x;
    let e = shifted_party1_y+shifted_party2_y;

    // finally calculate the shares for each party
    let share1 = p1_triple.c+ d*p1_triple.b + p1_triple.a*e +d*e;
    // no need to add the constant twice. other values have variable(shares) which will get added later
    let share2 = p2_triple.c+ d*p2_triple.b + p2_triple.a*e ;

    let reconstructed_value = share1+share2;
    let z = x*y;
    println!("Values x, y and z {:?} {:?} {:?}" , share1,share2,reconstructed_value);
    assert!(z==reconstructed_value);

    
    println!("==============================================================");
    println!("Verification of MAC and that the other parties are sharing correct alpha shares");

    // 1. Setup the global alpha and beta for each party(two party only for illustration)

    let alpha = FieldElement::new(50);
    // for each party one beta
    let betas:Vec<FieldElement> = vec![FieldElement::new(12), FieldElement::new(15)];
    let alpha_1 = FieldElement::new(15,);
    let alpha_2:FieldElement = alpha-alpha_1;
    let alphas = vec![alpha_1, alpha_2];
    
    // 2. Setup the alpha_beta for each and secret share between the parties (manually)
    // ideally, would like to call a function secret_share that would generate n-1 random shares
    // and n_th share such that it would hold summation with the secret
    let alpha_beta:Vec<FieldElement> = betas.iter().map(|beta| alpha*(*beta)).collect();
    let alpha_beta0_1 = FieldElement::new(123);
    let alpha_beta0_2 = alpha_beta[0]-alpha_beta0_1;
    let alpha_beta1_1 = FieldElement::new(527);
    let alpha_beta1_2 = alpha_beta[1]-alpha_beta1_1;

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
    party2.change_alpha(FieldElement::new(27));
    // Failing case 
    party_2_alpha = party2.broadcast_alpha_share().clone();
    let failing_status = party1.check_authenticity_of_reconstructed_alpha(&vec![party_1_alpha, party_2_alpha], &party1_alpha_beta);
    assert!(failing_status == false);
