///global impls
use crate::{
    app::App,
    common::{Float, Int, UInt},
};

pub trait TraitInto<T> {
    fn into_diy(self) -> T;
}

impl TraitInto<Int> for i64 {
    fn into_diy(self) -> Int {
        self as Int
    }
}

impl TraitInto<Float> for i64 {
    fn into_diy(self) -> Float {
        self as Float
    }
}

impl TraitInto<UInt> for i64 {
    fn into_diy(self) -> UInt {
        self as UInt
    }
}

pub trait TraitReverseInto<T> {
    fn into_std(self) -> T;
}

impl TraitReverseInto<i64> for Int {
    fn into_std(self) -> i64 {
        self as i64
    }
}

impl TraitReverseInto<u64> for UInt {
    fn into_std(self) -> u64 {
        self as u64
    }
}
