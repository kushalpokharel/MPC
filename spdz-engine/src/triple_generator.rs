use crate::field_element::FieldElement;

// A simple container for one party's share of a triple (a, b, c)
// Ideally, these would also include MACs for verification.
#[derive(Debug, Clone)]
pub struct TripleShare {
    pub a: FieldElement,
    pub b: FieldElement,
    pub c: FieldElement,
}

pub struct TripleGenerator {
    // In the future, this will hold the global MAC key 'alpha'
}

impl TripleGenerator {
    pub fn new() -> Self {
        TripleGenerator {}
    }

    /// Generates a valid multiplication triple and splits it between two parties.
    /// Returns: (Share for Party 1, Share for Party 2)
    pub fn generate_triple(&self) -> (TripleShare, TripleShare) {
        // 1. Generate random secret values (using hardcoded simple integers for now)
        let a_val = 5; 
        let b_val = 7;
        let c_val = a_val * b_val;

        let a = FieldElement::new(a_val);
        let b = FieldElement::new(b_val);
        let c = FieldElement::new(c_val);

        // 2. Split 'a' into shares
        // Party 1 gets random 'a1', Party 2 gets 'a2' such that a1 + a2 = a
        let (a1, a2) = self.share_value(&a);
        
        // 3. Split 'b' into shares
        let (b1, b2) = self.share_value(&b);

        // 4. Split 'c' into shares
        let (c1, c2) = self.share_value(&c);

        // 5. Bundle them
        let party1_share = TripleShare { a: a1, b: b1, c: c1 };
        let party2_share = TripleShare { a: a2, b: b2, c: c2 };

        (party1_share, party2_share)
    }

    // Helper to split a secret into two random shares
    fn share_value(&self, secret: &FieldElement) -> (FieldElement, FieldElement) {
        // For simulation, pick a random share for P1, calculate P2's share
        // P2 = Secret - P1
        let p1_val = 2; // constant for now. 
        let p1_share = FieldElement::new(p1_val);
        let p2_share = *secret - p1_share;
        
        (p1_share, p2_share)
    }
}