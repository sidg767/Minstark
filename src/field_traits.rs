#![allow(dead_code)]

use core::convert::TryFrom;
use core::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign, BitAnd, Div, DivAssign, Mul, MulAssign, Neg, Shl, Shr, Sub, SubAssign},
};
use std::vec::Vec;
pub trait FieldElement:
    Copy
    + Clone
    + Debug
    + Display
    + Default
    + Eq
    + PartialEq
    + Sized
    + Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Mul<Self, Output = Self>
    + Div<Self, Output = Self>
    + AddAssign<Self>
    + SubAssign<Self>
    + MulAssign<Self>
    + DivAssign<Self>
    + Neg<Output = Self>
    + From<u32>
    + From<u16>
    + From<u8>
    + TryFrom<u64>
    + TryFrom<u128>
{
    type PositiveInteger: Copy
        + PartialEq
        + PartialOrd
        + Shl<u32, Output = Self::PositiveInteger>
        + Shr<u32, Output = Self::PositiveInteger>
        + BitAnd<Output = Self::PositiveInteger>
        + From<u32>
        + From<u64>;

    type BaseField: FieldElement;

    const ZERO: Self;
    const ONE: Self;

    #[inline(always)]
    fn double(self) -> Self {
        self + self
    }

    #[inline(always)]
    fn square(self) -> Self {
        self * self
    }

    #[inline(always)]
    fn cube(self) -> Self {
        self * self * self
    }
    fn inv(self) -> Self;
    #[inline]
    fn pow(self, exp: u64) -> Self {
        let mut base = self;
        let mut e = exp;
        let mut result = Self::ONE;

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
pub trait StarkField: FieldElement<BaseField = Self> {
    const ELEMENT_BYTES: usize;
    const TWO_ADICITY: u32;
    const TWO_ADIC_ROOT_OF_UNITY: Self;

    const MODULUS: Self::PositiveInteger;
    const MODULUS_BITS: u32;
    const GENERATOR: Self;

    fn get_modulus_le_bytes() -> Vec<u8>;
    fn as_int(&self) -> Self::PositiveInteger;
    fn get_root_of_unity(n: u32) -> Self {
        assert!(n != 0, "n must be non-zero");
        assert!(n <= Self::TWO_ADICITY, "n exceeds field two-adicity");

        let power = 1u64 << (Self::TWO_ADICITY - n);
        Self::TWO_ADIC_ROOT_OF_UNITY.pow(power)
    }
    fn from_bytes_with_padding(bytes: &[u8]) -> Self
    where
        Self: From<u64>,
    {
        assert!(
            bytes.len() <= Self::ELEMENT_BYTES,
            "input too large for field element"
        );

        let mut acc = Self::from(0u64);
        for &b in bytes {
            acc = acc * Self::from(256u64) + Self::from(b as u64);
        }

        acc
    }
}

pub trait ExtensibleField<const N: usize>: StarkField {
    fn mul(a: [Self; N], b: [Self; N]) -> [Self; N];

    fn square(a: [Self; N]) -> [Self; N] {
        <Self as ExtensibleField<N>>::mul(a, a)
    }

    fn mul_base(a: [Self; N], b: Self) -> [Self; N];

    fn frobenius(x: [Self; N]) -> [Self; N];

    fn is_supported() -> bool {
        true
    }
}
pub trait ExtensionOf<E: FieldElement>: From<E> {
    fn mul_base(self, other: E) -> Self;
}
impl<E: FieldElement> ExtensionOf<E> for E {
    #[inline(always)]
    fn mul_base(self, other: E) -> Self {
        self * other
    }
}
pub trait ToElements<E: FieldElement> {
    fn to_elements(&self) -> Vec<E>;
}

impl<E: FieldElement> ToElements<E> for () {
    fn to_elements(&self) -> Vec<E> {
        Vec::new()
    }
}

impl<E: FieldElement> ToElements<E> for E {
    fn to_elements(&self) -> Vec<E> {
        vec![*self]
    }
}
