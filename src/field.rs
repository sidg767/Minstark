use crate::field_traits::{FieldElement, StarkField};
use core::convert::TryFrom;
use core::{
    fmt,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct BaseElement(pub u64);
impl fmt::Display for BaseElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl From<u8> for BaseElement {
    fn from(x: u8) -> Self {
        BaseElement::new(x as u64)
    }
}
impl From<u16> for BaseElement {
    fn from(x: u16) -> Self {
        BaseElement::new(x as u64)
    }
}
impl From<u32> for BaseElement {
    fn from(x: u32) -> Self {
        BaseElement::new(x as u64)
    }
}
impl TryFrom<u64> for BaseElement {
    type Error = &'static str;
    fn try_from(x: u64) -> Result<Self, Self::Error> {
        Ok(BaseElement::new(x))
    }
}
impl TryFrom<u128> for BaseElement {
    type Error = &'static str;
    fn try_from(x: u128) -> Result<Self, Self::Error> {
        if x <= u64::MAX as u128 {
            Ok(BaseElement::new(x as u64))
        } else {
            Err("value too large for BaseElement")
        }
    }
}
impl Add for BaseElement {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let sum = self.0 as u128 + rhs.0 as u128;
        BaseElement((sum % Self::MODULUS as u128) as u64)
    }
}

impl Sub for BaseElement {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        let diff = (self.0 as u128 + Self::MODULUS as u128 - rhs.0 as u128) % Self::MODULUS as u128;
        BaseElement(diff as u64)
    }
}

impl Mul for BaseElement {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        let product = self.0 as u128 * rhs.0 as u128;
        BaseElement((product % Self::MODULUS as u128) as u64)
    }
}

impl Div for BaseElement {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        self * rhs.inv()
    }
}
impl Neg for BaseElement {
    type Output = Self;
    fn neg(self) -> Self {
        BaseElement::ZERO - self
    }
}
impl AddAssign for BaseElement {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}
impl SubAssign for BaseElement {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}
impl MulAssign for BaseElement {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl DivAssign for BaseElement {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}
impl FieldElement for BaseElement {
    type PositiveInteger = u64;
    type BaseField = BaseElement;

    const ZERO: Self = BaseElement(0);
    const ONE: Self = BaseElement(1);

    fn inv(self) -> Self {
        assert!(self.0 != 0, "cannot invert zero");
        self.pow(Self::MODULUS - 2)
    }
}
impl StarkField for BaseElement {
    const MODULUS: Self::PositiveInteger = 18446744073709551557;
    const MODULUS_BITS: u32 = 64;

    const GENERATOR: Self = BaseElement(7);

    const TWO_ADICITY: u32 = 1;

    const TWO_ADIC_ROOT_OF_UNITY: Self = BaseElement(1);

    const ELEMENT_BYTES: usize = 8;

    fn get_modulus_le_bytes() -> Vec<u8> {
        Self::MODULUS.to_le_bytes().to_vec()
    }

    fn as_int(&self) -> Self::PositiveInteger {
        self.0
    }
}
pub type F = BaseElement;

impl BaseElement {
    #[inline]
    pub fn new(x: u64) -> Self {
        BaseElement(x % Self::MODULUS)
    }

    #[inline]
    pub fn as_int(&self) -> u64 {
        self.0
    }

    #[inline]
    pub fn pow(self, exp: u64) -> Self {
        let mut base = self;
        let mut e = exp;
        let mut result = BaseElement::ONE;

        while e > 0 {
            if e & 1 == 1 {
                result *= base;
            }
            base *= base;
            e >>= 1;
        }

        result
    }
}
#[test]
fn test_field_basic_arithmetic() {
    use crate::field::BaseElement as F;

    let a = F::new(7);
    let b = F::new(5);

    assert_eq!(a + b, F::new(12));
    assert_eq!(a * b, F::new(35));
    assert_eq!(a - b, F::new(2));
    assert_eq!(b.inv() * b, F::ONE);
}
