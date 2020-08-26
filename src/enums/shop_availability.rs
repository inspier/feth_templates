use std::convert::TryFrom;
use enum_default::EnumDefault;
use num_enum::TryFromPrimitive;

#[derive(Debug, TryFromPrimitive, EnumDefault)]
#[repr(u8)]
pub enum ShopAvailablity {
    Cannot_Buy = 0,
    Available = 1,
    Limited = 2,
    Unlimited = 3,
    Unreachable = u8::MAX,
}

binread_enum!(ShopAvailablity, u8);