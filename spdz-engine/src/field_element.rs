
use std::ops::{Add, Mul, Sub};
use std::iter::Sum;


pub type PRIMITIVE_TYPE = i32;
const MODULUS:i32 = 7919;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct FieldElement {
    value: PRIMITIVE_TYPE , 
}

impl FieldElement{
    pub fn new(value: PRIMITIVE_TYPE)->Self{
        FieldElement{
            value
        }
    }
}

impl Add<FieldElement> for FieldElement {
    type Output = FieldElement;
    fn add(self, other: FieldElement) -> FieldElement {
        FieldElement {
            value: (self.value + other.value) % MODULUS,
        }
    }
}

impl Sub<FieldElement> for FieldElement{
    type Output = FieldElement;
    fn sub(self, other: FieldElement) -> FieldElement {
        FieldElement {
            value: (self.value - other.value + MODULUS) % MODULUS,
        }
    }
}

impl Mul<FieldElement> for FieldElement{
    type Output = FieldElement;
    fn mul(self, other: FieldElement) -> FieldElement {
        FieldElement {
            value: (self.value * other.value) % MODULUS,
        }
    }
}

impl<'a> Sum<&'a FieldElement> for FieldElement {
    fn sum<I: Iterator<Item = &'a FieldElement>>(mut iter: I) -> Self {
        match iter.next() {
            None => FieldElement::new(0),
            Some(first) => {
                let init = FieldElement::new(first.value % MODULUS);
                iter.fold(init, |acc, x| acc + *x)
            }
        }
    }
}


