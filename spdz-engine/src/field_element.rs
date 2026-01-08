
use std::ops::{Add, Mul, Sub};
use std::iter::Sum;


pub type PRIMITIVE_TYPE = i32;
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct FieldElement {
    value: PRIMITIVE_TYPE , 
    modulus: PRIMITIVE_TYPE
}

impl FieldElement{
    pub fn new(value: PRIMITIVE_TYPE, modulus:PRIMITIVE_TYPE)->Self{
        FieldElement{
            value,
            modulus
        }
    }
}

impl<'a, 'b> Add<&'b FieldElement> for &'a FieldElement {
    type Output = FieldElement;
    fn add(self, other: &FieldElement) -> FieldElement {
        FieldElement {
            value: (self.value + other.value) % self.modulus,
            modulus: self.modulus,
        }
    }
}

impl<'a, 'b> Sub<&'b FieldElement> for &'a FieldElement{
    type Output = FieldElement;
    fn sub(self, other: &FieldElement) -> FieldElement {
        FieldElement {
            value: (self.value - other.value + self.modulus) % self.modulus,
            modulus: self.modulus,
        }
    }
}

impl<'a, 'b> Mul<&'b FieldElement> for &'a FieldElement{
    type Output = FieldElement;
    fn mul(self, other: &FieldElement) -> FieldElement {
        FieldElement {
            value: (self.value * other.value) % self.modulus,
            modulus: self.modulus,
        }
    }
}

impl<'a> Sum<&'a FieldElement> for FieldElement {
    fn sum<I: Iterator<Item = &'a FieldElement>>(mut iter: I) -> Self {
        match iter.next() {
            None => FieldElement::new(0, 1),
            Some(first) => {
                let init = FieldElement::new(first.value % first.modulus, first.modulus);
                iter.fold(init, |acc, x| &acc + x)
            }
        }
    }
}


