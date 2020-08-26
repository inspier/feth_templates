use deku::prelude::*;

#[derive(Debug, DekuRead)]
#[deku(id_type = "u8")]
pub enum EquipmentID {
    #[deku(id = "0")]
    Leather_Shield,
    #[deku(id = "1")]
    Iron_Shield,
    #[deku(id = "2")]
    Steel_Shield,
    #[deku(id = "3")]
    Silver_Shield,
    #[deku(id = "4")]
    Talisman_Shield,
    #[deku(id = "5")]
    Hexlock_Shield,
    #[deku(id = "6")]
    Aegis_Shield,
    #[deku(id = "7")]
    Ochain_Shield,
    #[deku(id = "8")]
    Seiros_Shield,
    #[deku(id = "9")]
    Aurora_Shield,
    #[deku(id = "10")]
    Kadmos_Shield,
    #[deku(id = "11")]
    Lampos_Shield,
    #[deku(id = "12")]
    Accuracy_Ring,
    #[deku(id = "13")]
    Critical_Ring,
    #[deku(id = "14")]
    Evasion_Ring,
    #[deku(id = "15")]
    Speed_Ring,
    #[deku(id = "16")]
    March_Ring,
    #[deku(id = "17")]
    Goddess_Ring,
    #[deku(id = "18")]
    Prayer_Ring,
    #[deku(id = "19")]
    Magic_Staff,
    #[deku(id = "20")]
    Healing_Staff,
    #[deku(id = "21")]
    Caduceus_Staff,
    #[deku(id = "22")]
    Thyrsus,
    #[deku(id = "23")]
    Rafail_Gem,
    #[deku(id = "24")]
    Experience_Gem,
    #[deku(id = "25")]
    Knowledge_Gem,
    #[deku(id = "26")]
    Circe_Staff,
    #[deku(id = "27")]
    Tomas_Staff,
    #[deku(id = "28")]
    Armored_Knight_Shield,
    #[deku(id = "29")]
    Great_Knight_Shield,
    #[deku(id = "30")]
    Fortress_Knight_Shield,
    #[deku(id = "31")]
    Asclepius,
    #[deku(id = "32")]
    Dark_Aegis_Shield,
    #[deku(id = "33")]
    Dark_Thyrsus,
    #[deku(id = "34")]
    Dark_Rafail_Gem,
    #[deku(id = "35")]
    Flame_Shield,
    #[deku(id = "36")]
    Emperor_Shield,
    #[deku(id = "37")]
    Black_Eagle_Pendant,
    #[deku(id = "38")]
    Blue_Lion_Brooch,
    #[deku(id = "39")]
    Golden_Deer_Bracelet,
    #[deku(id = "40")]
    White_Dragon_Scarf,
    #[deku(id = "41")]
    Equipment_41,
    #[deku(id = "42")]
    Equipment_42,
    #[deku(id = "43")]
    Equipment_43,
    #[deku(id = "44")]
    Equipment_44,
    #[deku(id = "45")]
    Equipment_45,
    #[deku(id = "46")]
    Equipment_46,
    #[deku(id = "47")]
    Equipment_47,
    #[deku(id = "48")]
    Equipment_48,
    #[deku(id = "49")]
    Equipment_49,
    #[deku(id = "50")]
    Equipment_50,
}
