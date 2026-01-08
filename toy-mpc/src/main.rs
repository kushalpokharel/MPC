

struct Party{
    uid: u32,
    share_x: FieldElement,
    share_y: FieldElement,
    share_a: FieldElement,
    share_b: FieldElement,
    share_c: FieldElement,
}
impl Party {
    fn new(uid:u32, share_x: FieldElement, share_y: FieldElement, share_a: FieldElement, share_b: FieldElement, share_c: FieldElement) -> Self {
        Party {
            uid,
            share_x,
            share_y,
            share_a,
            share_b,
            share_c,
        }
    }
    fn generate_and_broadcast_delta_share(&self)->FieldElement{
        // \delta = [x]-[a]. Since shares of a is completely random (for now it comes from the trusted source), 
        // delta doesn't leak information about x at all.
        // for broadcast we will append it to the global vector and access it later.(global mutable variable was difficult)
        // for broadcast just return to main which is our trusted source for now.
        return self.share_x.sub(&self.share_a);
    }
    fn generate_and_broadcast_epsilon_share(&self)->FieldElement{
        // \epsilon = [y]-[b]. Since shares of b is completely random (for now it comes from the trusted source), 
        // epsilon doesn't leak information about x at all.
        // for broadcast we will append it to the global vector and access it later.(global mutable variable was difficult)
        // for broadcast just return to main which is our trusted source for now.
        return self.share_y.sub(&self.share_b);
    }
    fn generate_multiplication_shares_locally(&self, epsilon:&FieldElement, delta:&FieldElement)->FieldElement{
        // x*y = (delta+a)(epsilon+b) = delta*epsilon+a*epsilon+b*delta+ab
        // epsilon, beta was calculated and provided. each party has the shares of a and b
        // a*b=c's shares are also present.
        let mut a = (&epsilon.mul(&self.share_a)).add(&delta.mul(&self.share_b)).add(&self.share_c);
        if self.uid==1{
            a = a.add(&delta.mul(&epsilon));
        }
        return a;

    }
}

#[derive(PartialEq, Debug)]
struct FieldElement {
    value: i32, 
    modulus: i32
}

impl FieldElement{
    fn new(value: i32, modulus:i32)->Self{
        FieldElement{
            value,
            modulus
        }
    }
}

trait FieldOps {
    fn add(&self, other: &FieldElement) -> FieldElement;
    fn sub(&self, other: &FieldElement) -> FieldElement;
    fn mul(&self, other: &FieldElement) -> FieldElement;
}

impl FieldOps for FieldElement {
    fn add(&self, other: &FieldElement) -> FieldElement {
        FieldElement {
            value: (self.value + other.value) % self.modulus,
            modulus: self.modulus,
        }
    }

    fn sub(&self, other: &FieldElement) -> FieldElement {
        FieldElement {
            value: (self.value - other.value + self.modulus) % self.modulus,
            modulus: self.modulus,
        }
    }

    fn mul(&self, other: &FieldElement) -> FieldElement {
        FieldElement {
            value: (self.value * other.value) % self.modulus,
            modulus: self.modulus,
        }
    }
}

fn main(){

    //We act as a trusted dealer here to distribute the shares to all the parties (two).
    let modulus:i32 = 2437;
    let x = FieldElement::new(5, modulus);
    let x1 = FieldElement::new(20, modulus);
    let x2 = x.sub(&x1);
    let y=FieldElement::new(78, modulus);
    let y1=FieldElement::new(44, modulus);
    let y2 = y.sub(&y1);
    // Beaver triples - a,b,c where a*b=c (25*4=100). Trusted generation.
    let a=FieldElement::new(25, modulus);
    let a1=FieldElement::new(12, modulus);
    let a2 = a.sub(&a1);
    let b=FieldElement::new(4, modulus);
    let b1=FieldElement::new(22, modulus);
    let b2 = b.sub(&b1);
    let c=FieldElement::new(100, modulus);
    let c1=FieldElement::new(44, modulus);
    let c2 = c.sub(&c1);
    // goal - to find the shares of z = x*y without communicating with other party 
    let party1 = Party::new(1,x1, y1, a1, b1, c1);
    let party2 = Party::new(2, x2,y2,a2,b2,c2);
    let epsilon1 = party1.generate_and_broadcast_epsilon_share();
    let epsilon2 = party2.generate_and_broadcast_epsilon_share();
    let delta1 = party1.generate_and_broadcast_delta_share();
    let delta2 = party2.generate_and_broadcast_delta_share();
    // epsilon = epsilon1+epsilon2 = y1-b1+y2-b2 = y-b
    // Similarly, beta = x-a
    let epsilon = epsilon1.add(&epsilon2);
    let delta = delta1.add(&delta2);

    // now finally we provide these values to each party and the parties can locally compute the shares of z
    let z1 = party1.generate_multiplication_shares_locally(&epsilon, &delta);
    let z2 = party2.generate_multiplication_shares_locally(&epsilon, &delta);
    let z = x.mul(&y);
    println!("Shares of z {:?} should be {:?} and {:?} {:?}", z, z1, z2, z1.add(&z2));
    assert!(z1.add(&z2) == z);
}


