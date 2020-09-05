use enum_default::EnumDefault;
use num_enum::TryFromPrimitive;
use std::convert::TryFrom;

#[derive(Debug, TryFromPrimitive, EnumDefault)]
#[repr(u8)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[allow(clippy::enum_variant_names)]
pub enum MerchantType {
    Southern_Merchant = 0x0,
    Eastern_Merchant = 0x1,
    Dark_Merchant = 0x2,
    No_Merchant = 0xFF,
}

binread_enum!(MerchantType, u8);
