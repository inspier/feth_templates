use deku::prelude::*;

#[derive(Debug, DekuRead)]
#[deku(id_type = "u8")]
pub enum ShopAvailablity {
    #[deku(id = "0")]
    Cannot_Buy,
    #[deku(id = "1")]
    Available,
    #[deku(id = "2")]
    Limited,
    #[deku(id_pat = "3")]
    Unlimited,
}
