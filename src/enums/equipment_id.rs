use enum_default::EnumDefault;
use num_enum::TryFromPrimitive;
use std::convert::TryFrom;

#[derive(Debug, TryFromPrimitive, EnumDefault)]
#[repr(u8)]
pub enum EquipmentID {
    Leather_Shield,
    Iron_Shield,
    Steel_Shield,
    Silver_Shield,
    Talisman_Shield,
    Hexlock_Shield,
    Aegis_Shield,
    Ochain_Shield,
    Seiros_Shield,
    Aurora_Shield,
    Kadmos_Shield,
    Lampos_Shield,
    Accuracy_Ring,
    Critical_Ring,
    Evasion_Ring,
    Speed_Ring,
    March_Ring,
    Goddess_Ring,
    Prayer_Ring,
    Magic_Staff,
    Healing_Staff,
    Caduceus_Staff,
    Thyrsus,
    Rafail_Gem,
    Experience_Gem,
    Knowledge_Gem,
    Circe_Staff,
    Tomas_Staff,
    Armored_Knight_Shield,
    Great_Knight_Shield,
    Fortress_Knight_Shield,
    Asclepius,
    Dark_Aegis_Shield,
    Dark_Thyrsus,
    Dark_Rafail_Gem,
    Flame_Shield,
    Emperor_Shield,
    Black_Eagle_Pendant,
    Blue_Lion_Brooch,
    Golden_Deer_Bracelet,
    White_Dragon_Scarf,
    Equipment_41,
    Equipment_42,
    Equipment_43,
    Equipment_44,
    Equipment_45,
    Equipment_46,
    Equipment_47,
    Equipment_48,
    Equipment_49,
    Equipment_50,
}

binread_enum!(EquipmentID, u8);
