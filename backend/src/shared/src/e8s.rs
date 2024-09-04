use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

use candid::{CandidType, Nat};
use lazy_static::lazy_static;
use num_bigint::BigUint;
use serde::Deserialize;

lazy_static! {
    pub static ref ES_BASES: [BigUint; 32] = {
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

pub type E8s = ECs<8>;

/// Fixed-point decimals with primitive math (+-*/) implemented correctly
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct ECs<const DECIMALS: usize> {
    pub val: BigUint,
}

impl<const D: usize> ECs<D> {
    pub fn new(val: BigUint) -> Self {
        Self { val }
    }

    pub fn base() -> &'static BigUint {
        // SAFETY: already checked
        unsafe { ES_BASES.get(D).unwrap_unchecked() }
    }

    pub fn base_d(decimals: u8) -> &'static BigUint {
        if decimals > 31 {
            unreachable!("Decimal points after 31 are not supported");
        }

        // SAFETY: already checked
        unsafe { ES_BASES.get(decimals as usize).unwrap_unchecked() }
    }

    pub fn zero() -> Self {
        Self::new(BigUint::ZERO)
    }

    pub fn one() -> Self {
        Self {
            val: Self::base().clone(),
        }
    }

    pub fn f0_1() -> Self {
        Self::new(Self::base() / BigUint::from(10u64))
    }

    pub fn f0_2() -> Self {
        Self::new(Self::base() / BigUint::from(5u64))
    }

    pub fn f0_25() -> Self {
        Self::new(Self::base() / BigUint::from(4u64))
    }

    pub fn f0_3() -> Self {
        Self::new(Self::base() * BigUint::from(3u64) / BigUint::from(10u64))
    }

    pub fn f0_33() -> Self {
        Self::new(Self::base() / BigUint::from(3u64))
    }

    pub fn f0_4() -> Self {
        Self::new(Self::base() * BigUint::from(2u64) / BigUint::from(5u64))
    }

    pub fn f0_5() -> Self {
        Self::new(Self::base() / BigUint::from(2u64))
    }

    pub fn f0_6() -> Self {
        Self::new(Self::base() * BigUint::from(3u64) / BigUint::from(5u64))
    }

    pub fn f0_67() -> Self {
        Self::new(Self::base() * BigUint::from(2u64) / BigUint::from(3u64))
    }

    pub fn f0_7() -> Self {
        Self::new(Self::base() * BigUint::from(7u64) / BigUint::from(10u64))
    }

    pub fn f0_75() -> Self {
        Self::new(Self::base() * BigUint::from(3u64) / BigUint::from(4u64))
    }

    pub fn f0_8() -> Self {
        Self::new(Self::base() * BigUint::from(4u64) / BigUint::from(5u64))
    }

    pub fn f0_9() -> Self {
        Self::new(Self::base() * BigUint::from(9u64) / BigUint::from(10u64))
    }

    pub fn two() -> Self {
        Self::new(Self::base() * BigUint::from(2u64))
    }

    pub fn sqrt(&self) -> Self {
        let base = Self::base();
        let whole = &self.val / base;
        let sqrt_whole = whole.sqrt();

        Self::new(sqrt_whole * base)
    }

    pub fn to_dynamic(self) -> EDs {
        EDs::new(self.val, D as u8)
    }

    pub fn to_decimals<const D1: usize>(self) -> ECs<D1> {
        if D1 == D {
            return ECs::<D1>::new(self.val);
        }

        let (dif, mul) = if D > D1 {
            (D - D1, false)
        } else {
            (D1 - D, true)
        };

        let base = Self::base_d(dif as u8);

        if mul {
            ECs::<D1>::new(self.val * base)
        } else {
            ECs::<D1>::new(self.val / base)
        }
    }
}

impl<const D: usize> Display for ECs<D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let base = ECs::<D>::base();

        f.write_str(&format!("{}.{}", &self.val / base, &self.val % base))
    }
}

impl<const D: usize> Add for &ECs<D> {
    type Output = ECs<D>;

    fn add(self, rhs: Self) -> Self::Output {
        ECs::<D>::new(&self.val + &rhs.val)
    }
}

impl<const D: usize> Add for ECs<D> {
    type Output = ECs<D>;

    fn add(self, rhs: Self) -> Self::Output {
        (&self).add(&rhs)
    }
}

impl<const D: usize> Add<&ECs<D>> for ECs<D> {
    type Output = ECs<D>;

    fn add(self, rhs: &ECs<D>) -> Self::Output {
        (&self).add(rhs)
    }
}

impl<const D: usize> Add<ECs<D>> for &ECs<D> {
    type Output = ECs<D>;

    fn add(self, rhs: ECs<D>) -> Self::Output {
        self.add(&rhs)
    }
}

impl<const D: usize> AddAssign<&ECs<D>> for ECs<D> {
    fn add_assign(&mut self, rhs: &ECs<D>) {
        self.val.add_assign(&rhs.val)
    }
}

impl<const D: usize> AddAssign for ECs<D> {
    fn add_assign(&mut self, rhs: Self) {
        self.add_assign(&rhs)
    }
}

impl<const D: usize> Sub for &ECs<D> {
    type Output = ECs<D>;

    fn sub(self, rhs: Self) -> Self::Output {
        ECs::<D>::new(&self.val - &rhs.val)
    }
}

impl<const D: usize> Sub for ECs<D> {
    type Output = ECs<D>;

    fn sub(self, rhs: Self) -> Self::Output {
        (&self).sub(&rhs)
    }
}

impl<const D: usize> Sub<&ECs<D>> for ECs<D> {
    type Output = ECs<D>;

    fn sub(self, rhs: &ECs<D>) -> Self::Output {
        (&self).sub(rhs)
    }
}

impl<const D: usize> Sub<ECs<D>> for &ECs<D> {
    type Output = ECs<D>;

    fn sub(self, rhs: ECs<D>) -> Self::Output {
        self.sub(&rhs)
    }
}

impl<const D: usize> SubAssign<&ECs<D>> for ECs<D> {
    fn sub_assign(&mut self, rhs: &ECs<D>) {
        self.val.sub_assign(&rhs.val)
    }
}

impl<const D: usize> SubAssign for ECs<D> {
    fn sub_assign(&mut self, rhs: Self) {
        self.sub_assign(&rhs)
    }
}

impl<const D: usize> Mul for &ECs<D> {
    type Output = ECs<D>;

    fn mul(self, rhs: Self) -> Self::Output {
        ECs::<D>::new(&self.val * &rhs.val / ECs::<D>::base())
    }
}

impl<const D: usize> Mul for ECs<D> {
    type Output = ECs<D>;

    fn mul(self, rhs: Self) -> Self::Output {
        (&self).mul(&rhs)
    }
}

impl<const D: usize> Mul<&ECs<D>> for ECs<D> {
    type Output = ECs<D>;

    fn mul(self, rhs: &ECs<D>) -> Self::Output {
        (&self).mul(rhs)
    }
}

impl<const D: usize> Mul<ECs<D>> for &ECs<D> {
    type Output = ECs<D>;

    fn mul(self, rhs: ECs<D>) -> Self::Output {
        self.mul(&rhs)
    }
}

impl<const D: usize> MulAssign<&ECs<D>> for ECs<D> {
    fn mul_assign(&mut self, rhs: &ECs<D>) {
        self.val = &self.val * &rhs.val / ECs::<D>::base()
    }
}

impl<const D: usize> MulAssign for ECs<D> {
    fn mul_assign(&mut self, rhs: Self) {
        self.mul_assign(&rhs)
    }
}

impl<const D: usize> Div for &ECs<D> {
    type Output = ECs<D>;

    fn div(self, rhs: Self) -> Self::Output {
        ECs::<D>::new(&self.val * ECs::<D>::base() / &rhs.val)
    }
}

impl<const D: usize> Div for ECs<D> {
    type Output = ECs<D>;

    fn div(self, rhs: Self) -> Self::Output {
        (&self).div(&rhs)
    }
}

impl<const D: usize> Div<&ECs<D>> for ECs<D> {
    type Output = ECs<D>;

    fn div(self, rhs: &ECs<D>) -> Self::Output {
        (&self).div(rhs)
    }
}

impl<const D: usize> Div<ECs<D>> for &ECs<D> {
    type Output = ECs<D>;

    fn div(self, rhs: ECs<D>) -> Self::Output {
        self.div(&rhs)
    }
}

impl<const D: usize> DivAssign<&ECs<D>> for ECs<D> {
    fn div_assign(&mut self, rhs: &ECs<D>) {
        self.val = &self.val * ECs::<D>::base() / &rhs.val;
    }
}

impl<const D: usize> DivAssign for ECs<D> {
    fn div_assign(&mut self, rhs: Self) {
        self.div_assign(&rhs)
    }
}

impl<const D: usize> CandidType for ECs<D> {
    fn _ty() -> candid::types::Type {
        Nat::_ty()
    }

    fn idl_serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: candid::types::Serializer,
    {
        Nat::idl_serialize(&Nat(self.val.clone()), serializer)
    }
}

impl<'de, const C: usize> Deserialize<'de> for ECs<C> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(ECs::new(Nat::deserialize(deserializer)?.0))
    }
}

impl<const D: usize> From<u64> for ECs<D> {
    fn from(value: u64) -> Self {
        Self::new(BigUint::from(value))
    }
}

/// Fixed-point decimals with primitive math (+-*/) implemented correctly
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct EDs {
    pub val: BigUint,
    pub decimals: u8,
}

impl EDs {
    pub fn new(val: BigUint, decimals: u8) -> Self {
        if decimals > 31 {
            unreachable!("Decimal points after 31 are not supported");
        }

        Self { val, decimals }
    }

    pub fn base(decimals: u8) -> &'static BigUint {
        if decimals > 31 {
            unreachable!("Decimal points after 31 are not supported");
        }

        // SAFETY: already checked
        unsafe { ES_BASES.get(decimals as usize).unwrap_unchecked() }
    }

    pub fn zero(decimals: u8) -> Self {
        Self::new(BigUint::ZERO, decimals)
    }

    pub fn one(decimals: u8) -> Self {
        Self {
            val: Self::base(decimals).clone(),
            decimals,
        }
    }

    pub fn f0_1(decimals: u8) -> Self {
        Self::new(Self::base(decimals) / BigUint::from(10u64), decimals)
    }

    pub fn f0_2(decimals: u8) -> Self {
        Self::new(Self::base(decimals) / BigUint::from(5u64), decimals)
    }

    pub fn f0_25(decimals: u8) -> Self {
        Self::new(Self::base(decimals) / BigUint::from(4u64), decimals)
    }

    pub fn f0_3(decimals: u8) -> Self {
        Self::new(
            Self::base(decimals) * BigUint::from(3u64) / BigUint::from(10u64),
            decimals,
        )
    }

    pub fn f0_33(decimals: u8) -> Self {
        Self::new(Self::base(decimals) / BigUint::from(3u64), decimals)
    }

    pub fn f0_4(decimals: u8) -> Self {
        Self::new(
            Self::base(decimals) * BigUint::from(2u64) / BigUint::from(5u64),
            decimals,
        )
    }

    pub fn f0_5(decimals: u8) -> Self {
        Self::new(Self::base(decimals) / BigUint::from(2u64), decimals)
    }

    pub fn f0_6(decimals: u8) -> Self {
        Self::new(
            Self::base(decimals) * BigUint::from(3u64) / BigUint::from(5u64),
            decimals,
        )
    }

    pub fn f0_67(decimals: u8) -> Self {
        Self::new(
            Self::base(decimals) * BigUint::from(2u64) / BigUint::from(3u64),
            decimals,
        )
    }

    pub fn f0_7(decimals: u8) -> Self {
        Self::new(
            Self::base(decimals) * BigUint::from(7u64) / BigUint::from(10u64),
            decimals,
        )
    }

    pub fn f0_75(decimals: u8) -> Self {
        Self::new(
            Self::base(decimals) * BigUint::from(3u64) / BigUint::from(4u64),
            decimals,
        )
    }

    pub fn f0_8(decimals: u8) -> Self {
        Self::new(
            Self::base(decimals) * BigUint::from(4u64) / BigUint::from(5u64),
            decimals,
        )
    }

    pub fn f0_9(decimals: u8) -> Self {
        Self::new(
            Self::base(decimals) * BigUint::from(9u64) / BigUint::from(10u64),
            decimals,
        )
    }

    pub fn two(decimals: u8) -> Self {
        Self::new(Self::base(decimals) * BigUint::from(2u64), decimals)
    }

    pub fn sqrt(&self) -> Self {
        let base = Self::base(self.decimals);
        let whole = &self.val / base;
        let sqrt_whole = whole.sqrt();

        Self::new(sqrt_whole * base, self.decimals)
    }

    pub fn to_const<const D: usize>(self) -> ECs<D> {
        if self.decimals != D as u8 {
            unreachable!(
                "{} decimals EDs can't be transformed into E{}s!",
                self.decimals, D
            );
        }

        ECs::new(self.val)
    }

    pub fn to_decimals(mut self, new_decimals: u8) -> EDs {
        if new_decimals == self.decimals {
            return self;
        }

        let (dif, mul) = if self.decimals > new_decimals {
            (self.decimals - new_decimals, false)
        } else {
            (new_decimals - self.decimals, true)
        };

        let base = Self::base(dif);
        self.val = if mul {
            self.val * base
        } else {
            self.val / base
        };

        self.decimals = new_decimals;

        self
    }
}

impl Display for EDs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let base = Self::base(self.decimals);

        f.write_str(&format!("{}.{}", &self.val / base, &self.val % base))
    }
}

impl Add for &EDs {
    type Output = EDs;

    fn add(self, rhs: Self) -> Self::Output {
        if self.decimals != rhs.decimals {
            unreachable!("Incompatible decimal points");
        }

        EDs::new(&self.val + &rhs.val, self.decimals)
    }
}

impl Add for EDs {
    type Output = EDs;

    fn add(self, rhs: Self) -> Self::Output {
        (&self).add(&rhs)
    }
}

impl Add<&EDs> for EDs {
    type Output = EDs;

    fn add(self, rhs: &EDs) -> Self::Output {
        (&self).add(rhs)
    }
}

impl Add<EDs> for &EDs {
    type Output = EDs;

    fn add(self, rhs: EDs) -> Self::Output {
        self.add(&rhs)
    }
}

impl AddAssign<&EDs> for EDs {
    fn add_assign(&mut self, rhs: &EDs) {
        if self.decimals != rhs.decimals {
            unreachable!("Incompatible decimal points");
        }

        self.val.add_assign(&rhs.val)
    }
}

impl AddAssign for EDs {
    fn add_assign(&mut self, rhs: Self) {
        self.add_assign(&rhs)
    }
}

impl Sub for &EDs {
    type Output = EDs;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.decimals != rhs.decimals {
            unreachable!("Incompatible decimal points");
        }

        EDs::new(&self.val - &rhs.val, self.decimals)
    }
}

impl Sub for EDs {
    type Output = EDs;

    fn sub(self, rhs: Self) -> Self::Output {
        (&self).sub(&rhs)
    }
}

impl Sub<&EDs> for EDs {
    type Output = EDs;

    fn sub(self, rhs: &EDs) -> Self::Output {
        (&self).sub(rhs)
    }
}

impl Sub<EDs> for &EDs {
    type Output = EDs;

    fn sub(self, rhs: EDs) -> Self::Output {
        self.sub(&rhs)
    }
}

impl SubAssign<&EDs> for EDs {
    fn sub_assign(&mut self, rhs: &EDs) {
        if self.decimals != rhs.decimals {
            unreachable!("Incompatible decimal points");
        }

        self.val.sub_assign(&rhs.val)
    }
}

impl SubAssign for EDs {
    fn sub_assign(&mut self, rhs: Self) {
        self.sub_assign(&rhs)
    }
}

impl Mul for &EDs {
    type Output = EDs;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.decimals != rhs.decimals {
            unreachable!("Incompatible decimal points");
        }

        EDs::new(
            &self.val * &rhs.val / EDs::base(self.decimals),
            self.decimals,
        )
    }
}

impl Mul for EDs {
    type Output = EDs;

    fn mul(self, rhs: Self) -> Self::Output {
        (&self).mul(&rhs)
    }
}

impl Mul<&EDs> for EDs {
    type Output = EDs;

    fn mul(self, rhs: &EDs) -> Self::Output {
        (&self).mul(rhs)
    }
}

impl Mul<EDs> for &EDs {
    type Output = EDs;

    fn mul(self, rhs: EDs) -> Self::Output {
        self.mul(&rhs)
    }
}

impl Mul<u64> for &EDs {
    type Output = EDs;

    fn mul(self, rhs: u64) -> Self::Output {
        EDs::new(&self.val * BigUint::from(rhs), self.decimals)
    }
}

impl Mul<u64> for EDs {
    type Output = EDs;

    fn mul(self, rhs: u64) -> Self::Output {
        EDs::new(self.val * BigUint::from(rhs), self.decimals)
    }
}

impl MulAssign<&EDs> for EDs {
    fn mul_assign(&mut self, rhs: &EDs) {
        if self.decimals != rhs.decimals {
            unreachable!("Incompatible decimal points");
        }

        self.val = &self.val * &rhs.val / EDs::base(self.decimals)
    }
}

impl MulAssign for EDs {
    fn mul_assign(&mut self, rhs: Self) {
        self.mul_assign(&rhs)
    }
}

impl MulAssign<u64> for EDs {
    fn mul_assign(&mut self, rhs: u64) {
        self.val *= BigUint::from(rhs);
    }
}

impl Div for &EDs {
    type Output = EDs;

    fn div(self, rhs: Self) -> Self::Output {
        if self.decimals != rhs.decimals {
            unreachable!("Incompatible decimal points");
        }

        EDs::new(
            &self.val * EDs::base(self.decimals) / &rhs.val,
            self.decimals,
        )
    }
}

impl Div for EDs {
    type Output = EDs;

    fn div(self, rhs: Self) -> Self::Output {
        (&self).div(&rhs)
    }
}

impl Div<u64> for EDs {
    type Output = EDs;

    fn div(self, rhs: u64) -> Self::Output {
        EDs::new(self.val / BigUint::from(rhs), self.decimals)
    }
}

impl Div<u64> for &EDs {
    type Output = EDs;

    fn div(self, rhs: u64) -> Self::Output {
        EDs::new(&self.val / BigUint::from(rhs), self.decimals)
    }
}

impl Div<&EDs> for EDs {
    type Output = EDs;

    fn div(self, rhs: &EDs) -> Self::Output {
        (&self).div(rhs)
    }
}

impl Div<EDs> for &EDs {
    type Output = EDs;

    fn div(self, rhs: EDs) -> Self::Output {
        self.div(&rhs)
    }
}

impl DivAssign<&EDs> for EDs {
    fn div_assign(&mut self, rhs: &EDs) {
        if self.decimals != rhs.decimals {
            unreachable!("Incompatible decimal points");
        }

        *(&mut self.val) = &self.val * EDs::base(self.decimals) / &rhs.val;
    }
}

impl DivAssign for EDs {
    fn div_assign(&mut self, rhs: Self) {
        self.div_assign(&rhs)
    }
}

impl DivAssign<u64> for EDs {
    fn div_assign(&mut self, rhs: u64) {
        self.val /= BigUint::from(rhs);
    }
}

impl From<(u64, u8)> for EDs {
    fn from((value, decimals): (u64, u8)) -> Self {
        Self::new(BigUint::from(value), decimals)
    }
}

impl Into<Nat> for EDs {
    fn into(self) -> Nat {
        Nat(self.val)
    }
}

impl Into<Nat> for &EDs {
    fn into(self) -> Nat {
        Nat(self.val.clone())
    }
}

#[derive(CandidType, Deserialize)]
pub struct EDsCandid {
    pub val: Nat,
    pub decimals: u8,
}

impl CandidType for EDs {
    fn _ty() -> candid::types::Type {
        EDsCandid::_ty()
    }

    fn idl_serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: candid::types::Serializer,
    {
        (EDsCandid {
            val: Nat(self.val.clone()),
            decimals: self.decimals,
        })
        .idl_serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for EDs {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let a = EDsCandid::deserialize(deserializer)?;

        Ok(Self::new(a.val.0, a.decimals))
    }
}
