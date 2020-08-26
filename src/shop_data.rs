use deku::prelude::*;
use crate::enums::{
    equipment_id::EquipmentID, misc_item::MiscItem, shop_availability::ShopAvailablity,
    weapon_id::WeaponID,
};

#[derive(Debug, DekuRead)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[allow(clippy::enum_variant_names)]
#[deku(id_type = "u8")]
enum MerchantType {
    #[deku(id = "0x0")]
    Southern_Merchant = 0x0,
    #[deku(id = "0x1")]
    Eastern_Merchant = 0x1,
    #[deku(id = "0x2")]
    Dark_Merchant = 0x2,
    #[deku(id = "0xFF")]
    No_Merchant = 0xFF,
}

#[derive(Debug, DekuRead)]
#[deku(ctx = "_: deku::ctx::Endian")]
struct SectionPointers {
    header: u32,
    pointer_to_weapon_table: u32,
    size_of_weapon_table: u32,
    pointer_to_equipment_table: u32,
    size_of_equipment_table: u32,
    pointer_to_item_table: u32,
    size_of_item_table: u32,
    pointer_to_table_04: u32,
    size_of_table_04: u32,
    pointer_to_table_05: u32,
    size_of_table_05: u32,
    pointer_to_table_06: u32,
    size_of_table_06: u32,
    pointer_to_table_07: u32,
    size_of_table_07: u32,
    pointer_to_table_08: u32,
    size_of_table_08: u32,
}

#[derive(Debug, DekuRead)]
struct WeaponTableHeader {
    header: u32,
    num_of_weapon: u32,
    size_of_weapon: u32,
    #[deku(count = "0x34")]
    padding: Vec<i8>,
}

#[derive(Debug, DekuRead)]
struct WeaponTable {
    buy_price: i32,
    sell_price: i32,
    weapon_id: WeaponID,
    unk0: u8,
    unk1: u8,
    availablity: ShopAvailablity,
    repair_material: MiscItem,
    repair_cost: u8,
    unk2: u8,
    store_lv1_stock: i8,
    store_lv2_stock: i8,
    store_lv3_stock: i8,
    padding: u8,
}

#[derive(Debug, DekuRead)]
#[deku(ctx = "_: deku::ctx::Endian")]
struct WeaponTableStructure {
    weapon_table_header: WeaponTableHeader,
    #[deku(count = "weapon_table_header.num_of_weapon")]
    weapon_shop_data: Vec<WeaponTable>,
}

#[derive(Debug, DekuRead)]
struct EquipmentTableHeader {
    header: u32,
    num_of_equipment: u32,
    size_of_equipment: u32,
    #[deku(count = "0x34")]
    padding: Vec<i8>,
}

#[derive(Debug, DekuRead)]
struct EquipmentTable {
    buy_price: i32,
    sell_price: i32,
    equipment_id: EquipmentID,
    id_padding: u8,
    unk0: u8,
    unk1: u8,
    availablity: ShopAvailablity,
    unk2: u8,
    unk3: u8,
    unk4: u8,
}

#[derive(Debug, DekuRead)]
#[deku(ctx = "_: deku::ctx::Endian")]
struct EquipmentTableStructure {
    equipment_table_header: EquipmentTableHeader,
    #[deku(count = "equipment_table_header.num_of_equipment")]
    equipment_table_data: Vec<EquipmentTable>,
}

#[derive(Debug, DekuRead)]
#[deku(endian = "little")]
pub struct File {
    section_pointer: SectionPointers,
    #[deku(count = "0x28")]
    unused: Vec<u8>,
    weapon_shop_table: WeaponTableStructure,
    equipment_table: EquipmentTableStructure,
}