use num_enum::TryFromPrimitive;
use std::convert::TryFrom;

#[derive(Debug, TryFromPrimitive)]
#[repr(u8)]
pub enum ShopAvailablity {
    Cannot_Buy = 0,
    Available = 1,
    Limited = 2,
    Unlimited = 3,
}

binread_enum!(ShopAvailablity, u8);
