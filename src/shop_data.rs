use crate::enums::{
    equipment_id::EquipmentID, general_merchant_items::GeneralMerchantItems, item_id::ItemID,
    merchant_type::MerchantType, misc_item::MiscItem, ranks::MinRanks,
    shop_availability::ShopAvailablity, weapon_id::WeaponID,
};
use binread::{io::SeekFrom, BinRead};

#[derive(Debug, BinRead)]
#[br(little)]
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

#[derive(Debug, BinRead)]
#[br(little)]
struct Header {
    header: u32,
    number_of: u32,
    size_of: u32,
    #[br(count = 0x34)]
    padding: Vec<i8>,
}

#[derive(Debug, BinRead)]
#[br(little)]
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

#[derive(Debug, BinRead)]
#[br(little)]
struct WeaponTableStructure {
    weapon_table_header: Header,
    #[br(count = weapon_table_header.number_of)]
    weapon_shop_data: Vec<WeaponTable>,
}

#[derive(Debug, BinRead)]
#[br(little)]
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

#[derive(Debug, BinRead)]
#[br(little)]
struct EquipmentTableStructure {
    equipment_table_header: Header,
    #[br(count = equipment_table_header.number_of)]
    equipment_table_data: Vec<EquipmentTable>,
}

#[derive(Debug, BinRead)]
#[br(little)]
struct ItemTable {
    buy_price: i32,
    sell_price: i32,
    item_id: ItemID,
    id_padding: u8,
    unk0: u8,
    unk1: u8,
    availablity: ShopAvailablity,
    unk2: u8,
    unk3: u8,
    unk4: u8,
}

#[derive(Debug, BinRead)]
#[br(little)]
struct ItemTableStructure {
    item_table_header: Header,
    #[br(count = item_table_header.number_of)]
    item_table_data: Vec<ItemTable>,
}

#[derive(Debug, BinRead)]
struct Table04 {
    #[br(count = 0x14)]
    table04_size: Vec<u8>,
}

#[derive(Debug, BinRead)]
struct Table04Structure {
    header: u32,
    num_of_table04: u32,
    size_of_table04: u32,
    #[br(count = 0x34)]
    padding: Vec<i8>,
    #[br(count = num_of_table04)]
    table04: Vec<Table04>,
}

#[derive(Debug, BinRead)]
struct Table05 {
    price: i32,
    base_weapon: WeaponID,
    forge_material: MiscItem,
    item_byte_padding: u8,
    forged_weapon: WeaponID,
    required_professor_level: MinRanks,
    required_material_amount: u8,
}

#[derive(Debug, BinRead)]
struct Table05Structure {
    header: u32,
    num_of_table05: u32,
    size_of_table05: u32,
    #[br(count = 0x34)]
    padding: Vec<i8>,
    #[br(count = num_of_table05)]
    table04: Vec<Table05>,
}

#[derive(Debug, BinRead)]
struct Table06 {
    #[br(count = 0x20)]
    table06_size: Vec<u8>,
}

#[derive(Debug, BinRead)]
struct Table06Structure {
    header: u32,
    num_of_table06: u32,
    size_of_table06: u32,
    #[br(count = 0x34)]
    padding: Vec<i8>,
    #[br(count = num_of_table06)]
    table06: Vec<Table06>,
}

#[derive(Debug, BinRead)]
struct Table07 {
    item_for_sale: GeneralMerchantItems,
    merchant: MerchantType,
    unk0: u8,
    availablity: ShopAvailablity,
    stock: u8,
}

#[derive(Debug, BinRead)]
struct Table07Structure {
    header: u32,
    num_of_table07: u32,
    size_of_table07: u32,
    #[br(count = 0x34)]
    padding: Vec<i8>,
    #[br(count = num_of_table07)]
    general_merchants_data: Vec<Table07>,
}

#[derive(Debug, BinRead)]
struct Table08 {
    price: i32,
    anna_item_id: GeneralMerchantItems,
    availablity: ShopAvailablity,
    stock: u8,
}

#[derive(Debug, BinRead)]
struct Table08Structure {
    header: u32,
    num_of_table08: u32,
    size_of_table08: u32,
    #[br(count = 0x34)]
    padding: Vec<i8>,
    #[br(count = num_of_table08)]
    anna_shop_data: Vec<Table08>,
}

#[derive(Debug, BinRead)]
#[br(little)]
pub struct File {
    section_pointer: SectionPointers,
    #[br(seek_before = SeekFrom::Start(section_pointer.pointer_to_weapon_table as u64))]
    weapon_shop_table: WeaponTableStructure,
    #[br(seek_before = SeekFrom::Start(section_pointer.pointer_to_equipment_table as u64))]
    equipment_table: EquipmentTableStructure,
    #[br(seek_before = SeekFrom::Start(section_pointer.pointer_to_item_table as u64))]
    item_table: ItemTableStructure,
    #[br(seek_before = SeekFrom::Start(u64::from(section_pointer.pointer_to_table_04)))]
    battalion_block: Table04Structure,
    #[br(seek_before = SeekFrom::Start(u64::from(section_pointer.pointer_to_table_05)))]
    forging_block: Table05Structure,
    #[br(seek_before = SeekFrom::Start(u64::from(section_pointer.pointer_to_table_06)))]
    table06_block: Table06Structure,
    #[br(seek_before = SeekFrom::Start(u64::from(section_pointer.pointer_to_table_07)))]
    general_merchants_block: Table07Structure,
    #[br(seek_before = SeekFrom::Start(u64::from(section_pointer.pointer_to_table_08)))]
    anna_shop_block: Table08Structure,
}
