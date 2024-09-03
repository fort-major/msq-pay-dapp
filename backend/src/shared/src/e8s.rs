use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, Sub, SubAssign},
};

use candid::{CandidType, Nat};
use lazy_static::lazy_static;
use num_bigint::BigUint;
use serde::Deserialize;

lazy_static! {
    pub static ref E8S_BASES: [BigUint; 32] = {
        let ten = BigUint::from(10u64);

        [
            ten.pow(0),
            ten.pow(1),
            ten.pow(2),
            ten.pow(3),
            ten.pow(4),
            ten.pow(5),
            ten.pow(6),
            ten.pow(7),
            ten.pow(8),
            ten.pow(9),
            ten.pow(10),
            ten.pow(11),
            ten.pow(12),
            ten.pow(13),
            ten.pow(14),
            ten.pow(15),
            ten.pow(16),
            ten.pow(17),
            ten.pow(18),
            ten.pow(19),
            ten.pow(20),
            ten.pow(21),
            ten.pow(22),
            ten.pow(23),
            ten.pow(24),
            ten.pow(25),
            ten.pow(26),
            ten.pow(27),
            ten.pow(28),
            ten.pow(29),
            ten.pow(30),
            ten.pow(31),
        ]
    };
}

/// Fixed-point decimals with primitive math (+-*/) implemented correctly
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct E8s {
    pub val: BigUint,
    pub decimals: u8,
}

#[derive(CandidType, Deserialize)]
pub struct E8sCandid {
    pub val: Nat,
    pub decimals: u8,
}

impl CandidType for E8s {
    fn _ty() -> candid::types::Type {
        E8sCandid::_ty()
    }

    fn idl_serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: candid::types::Serializer,
    {
        (E8sCandid {
            val: Nat(self.val.clone()),
            decimals: self.decimals,
        })
        .idl_serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for E8s {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let a = E8sCandid::deserialize(deserializer)?;

        Ok(Self::new_d(a.val.0, a.decimals))
    }
}

impl E8s {
    pub fn new_d(val: BigUint, decimals: u8) -> Self {
        if decimals > 31 {
            unreachable!("Decimal points after 31 are not supported");
        }

        Self { val, decimals }
    }

    pub fn new(val: BigUint) -> Self {
        Self::new_d(val, 8)
    }

    pub fn base_d(decimals: u8) -> &'static BigUint {
        if decimals > 31 {
            unreachable!("Decimal points after 31 are not supported");
        }

        // SAFETY: already checked
        unsafe { E8S_BASES.get(decimals as usize).unwrap_unchecked() }
    }

    pub fn base() -> &'static BigUint {
        Self::base_d(8)
    }

    pub fn zero_d(decimals: u8) -> Self {
        Self::new_d(BigUint::ZERO, decimals)
    }

    pub fn zero() -> Self {
        Self::new_d(BigUint::ZERO, 8)
    }

    pub fn one_d(decimals: u8) -> Self {
        Self {
            val: Self::base_d(decimals).clone(),
            decimals,
        }
    }

    pub fn one() -> Self {
        Self::one_d(8)
    }

    pub fn f0_1_d(decimals: u8) -> Self {
        Self::new_d(Self::base_d(decimals) / BigUint::from(10u64), decimals)
    }

    pub fn f0_1() -> Self {
        Self::f0_1_d(8)
    }

    pub fn f0_2_d(decimals: u8) -> Self {
        Self::new_d(Self::base_d(decimals) / BigUint::from(5u64), decimals)
    }

    pub fn f0_2() -> Self {
        Self::f0_2_d(8)
    }

    pub fn f0_25_d(decimals: u8) -> Self {
        Self::new_d(Self::base_d(decimals) / BigUint::from(4u64), decimals)
    }

    pub fn f0_25() -> Self {
        Self::f0_25_d(8)
    }

    pub fn f0_3_d(decimals: u8) -> Self {
        Self::new_d(
            Self::base_d(decimals) * BigUint::from(3u64) / BigUint::from(10u64),
            decimals,
        )
    }

    pub fn f0_3() -> Self {
        Self::f0_3_d(8)
    }

    pub fn f0_33_d(decimals: u8) -> Self {
        Self::new_d(Self::base_d(decimals) / BigUint::from(3u64), decimals)
    }

    pub fn f0_33() -> Self {
        Self::f0_33_d(8)
    }

    pub fn f0_4_d(decimals: u8) -> Self {
        Self::new_d(
            Self::base_d(decimals) * BigUint::from(2u64) / BigUint::from(5u64),
            decimals,
        )
    }

    pub fn f0_4() -> Self {
        Self::f0_4_d(8)
    }

    pub fn f0_5_d(decimals: u8) -> Self {
        Self::new_d(Self::base_d(decimals) / BigUint::from(2u64), decimals)
    }

    pub fn f0_5() -> Self {
        Self::f0_5_d(8)
    }

    pub fn f0_6_d(decimals: u8) -> Self {
        Self::new_d(
            Self::base_d(decimals) * BigUint::from(3u64) / BigUint::from(5u64),
            decimals,
        )
    }

    pub fn f0_6() -> Self {
        Self::f0_6_d(8)
    }

    pub fn f0_67_d(decimals: u8) -> Self {
        Self::new_d(
            Self::base_d(decimals) * BigUint::from(2u64) / BigUint::from(3u64),
            decimals,
        )
    }

    pub fn f0_67() -> Self {
        Self::f0_67_d(8)
    }

    pub fn f0_7_d(decimals: u8) -> Self {
        Self::new_d(
            Self::base_d(decimals) * BigUint::from(7u64) / BigUint::from(10u64),
            decimals,
        )
    }

    pub fn f0_7() -> Self {
        Self::f0_7_d(8)
    }

    pub fn f0_75_d(decimals: u8) -> Self {
        Self::new_d(
            Self::base_d(decimals) * BigUint::from(3u64) / BigUint::from(4u64),
            decimals,
        )
    }

    pub fn f0_75() -> Self {
        Self::f0_75_d(8)
    }

    pub fn f0_8_d(decimals: u8) -> Self {
        Self::new_d(
            Self::base_d(decimals) * BigUint::from(4u64) / BigUint::from(5u64),
            decimals,
        )
    }

    pub fn f0_8() -> Self {
        Self::f0_8_d(8)
    }

    pub fn f0_9_d(decimals: u8) -> Self {
        Self::new_d(
            Self::base_d(decimals) * BigUint::from(9u64) / BigUint::from(10u64),
            decimals,
        )
    }

    pub fn f0_9() -> Self {
        Self::f0_9_d(8)
    }

    pub fn two_d(decimals: u8) -> Self {
        Self::new_d(Self::base_d(decimals) * BigUint::from(2u64), decimals)
    }

    pub fn two() -> Self {
        Self::two_d(8)
    }

    pub fn sqrt(&self) -> Self {
        let base = Self::base_d(self.decimals);
        let whole = &self.val / base;
        let sqrt_whole = whole.sqrt();

        Self::new_d(sqrt_whole * base, self.decimals)
    }

    pub fn to_precision_2(self) -> E8s {
        if self.decimals < 2 {
            unreachable!("Precision 2 can only happen for decimals >= 2");
        }

        let base = Self::base_d(self.decimals - 2);

        Self::new_d(self.val / base * base, self.decimals)
    }
}

impl Display for E8s {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let base = Self::base_d(self.decimals);

        f.write_str(&format!("{}.{}", &self.val / base, &self.val % base))
    }
}

impl Add for &E8s {
    type Output = E8s;

    fn add(self, rhs: Self) -> Self::Output {
        if self.decimals != rhs.decimals {
            unreachable!("Incompatible decimal points");
        }

        E8s::new_d(&self.val + &rhs.val, self.decimals)
    }
}

impl Add for E8s {
    type Output = E8s;

    fn add(self, rhs: Self) -> Self::Output {
        (&self).add(&rhs)
    }
}

impl Add<&E8s> for E8s {
    type Output = E8s;

    fn add(self, rhs: &E8s) -> Self::Output {
        (&self).add(rhs)
    }
}

impl Add<E8s> for &E8s {
    type Output = E8s;

    fn add(self, rhs: E8s) -> Self::Output {
        self.add(&rhs)
    }
}

impl AddAssign<&E8s> for E8s {
    fn add_assign(&mut self, rhs: &E8s) {
        if self.decimals != rhs.decimals {
            unreachable!("Incompatible decimal points");
        }

        self.val.add_assign(&rhs.val)
    }
}

impl AddAssign for E8s {
    fn add_assign(&mut self, rhs: Self) {
        self.add_assign(&rhs)
    }
}

impl Sub for &E8s {
    type Output = E8s;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.decimals != rhs.decimals {
            unreachable!("Incompatible decimal points");
        }

        E8s::new_d(&self.val - &rhs.val, self.decimals)
    }
}

impl Sub for E8s {
    type Output = E8s;

    fn sub(self, rhs: Self) -> Self::Output {
        (&self).sub(&rhs)
    }
}

impl Sub<&E8s> for E8s {
    type Output = E8s;

    fn sub(self, rhs: &E8s) -> Self::Output {
        (&self).sub(rhs)
    }
}

impl Sub<E8s> for &E8s {
    type Output = E8s;

    fn sub(self, rhs: E8s) -> Self::Output {
        self.sub(&rhs)
    }
}

impl SubAssign<&E8s> for E8s {
    fn sub_assign(&mut self, rhs: &E8s) {
        if self.decimals != rhs.decimals {
            unreachable!("Incompatible decimal points");
        }

        self.val.sub_assign(&rhs.val)
    }
}

impl SubAssign for E8s {
    fn sub_assign(&mut self, rhs: Self) {
        self.sub_assign(&rhs)
    }
}

impl Mul for &E8s {
    type Output = E8s;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.decimals != rhs.decimals {
            unreachable!("Incompatible decimal points");
        }

        E8s::new_d(
            &self.val * &rhs.val / E8s::base_d(self.decimals),
            self.decimals,
        )
    }
}

impl Mul for E8s {
    type Output = E8s;

    fn mul(self, rhs: Self) -> Self::Output {
        (&self).mul(&rhs)
    }
}

impl Mul<&E8s> for E8s {
    type Output = E8s;

    fn mul(self, rhs: &E8s) -> Self::Output {
        (&self).mul(rhs)
    }
}

impl Mul<E8s> for &E8s {
    type Output = E8s;

    fn mul(self, rhs: E8s) -> Self::Output {
        self.mul(&rhs)
    }
}

impl MulAssign<&E8s> for E8s {
    fn mul_assign(&mut self, rhs: &E8s) {
        if self.decimals != rhs.decimals {
            unreachable!("Incompatible decimal points");
        }

        *(&mut self.val) = &self.val * &rhs.val / E8s::base_d(self.decimals)
    }
}

impl MulAssign for E8s {
    fn mul_assign(&mut self, rhs: Self) {
        self.mul_assign(&rhs)
    }
}

impl Div for &E8s {
    type Output = E8s;

    fn div(self, rhs: Self) -> Self::Output {
        if self.decimals != rhs.decimals {
            unreachable!("Incompatible decimal points");
        }

        E8s::new_d(
            &self.val * E8s::base_d(self.decimals) / &rhs.val,
            self.decimals,
        )
    }
}

impl Div for E8s {
    type Output = E8s;

    fn div(self, rhs: Self) -> Self::Output {
        (&self).div(&rhs)
    }
}

impl Div<&E8s> for E8s {
    type Output = E8s;

    fn div(self, rhs: &E8s) -> Self::Output {
        (&self).div(rhs)
    }
}

impl Div<E8s> for &E8s {
    type Output = E8s;

    fn div(self, rhs: E8s) -> Self::Output {
        self.div(&rhs)
    }
}

impl DivAssign<&E8s> for E8s {
    fn div_assign(&mut self, rhs: &E8s) {
        if self.decimals != rhs.decimals {
            unreachable!("Incompatible decimal points");
        }

        *(&mut self.val) = &self.val * E8s::base_d(self.decimals) / &rhs.val;
    }
}

impl DivAssign for E8s {
    fn div_assign(&mut self, rhs: Self) {
        self.div_assign(&rhs)
    }
}
