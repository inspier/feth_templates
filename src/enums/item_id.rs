use enum_default::EnumDefault;
use num_enum::TryFromPrimitive;
use std::convert::TryFrom;

#[derive(Debug, TryFromPrimitive, EnumDefault)]
#[repr(u8)]
pub enum ItemID {
    Vulnerary,
    Concoction,
    Elixir,
    Intermediate_Seal = 3,
    Advanced_Seal = 4,
    Combined_Seal = 5,
    Master_Seal = 6,
    Torch_Item,
    Bullion,
    Large_Bullion,
    Extra_Large_Bullion,
    Antitoxin,
    Pure_Water,
    Door_Key,
    Chest_Key,
    Master_Key,
    Seraph_Robe,
    Energy_Drop,
    Spirit_Dust,
    Secret_Book,
    Speed_Wing,
    Goddess_Icon,
    Giant_Shell,
    Talisman,
    Black_Pearl,
    Shoes_of_the_Wind,
    Grilled_Duck,
    Fried_Boar,
    Magical_Herb_Salad,
    Dried_Plums,
    Pike_Casserole,
    Bear_and_Vegetable_Soup,
    Fried_Gar,
    Fried_Queen_Koi,
    Sculpin_Gratin,
    Grilled_Wolverine_and_Herbs,
    Grilled_Duck_plus,
    Fried_Boar_plus,
    Magical_Herb_Salad_plus,
    Dried_Plums_plus,
    Pike_Casserole_plus,
    Bear_and_Vegetable_Soup_plus,
    Fried_Gar_plus,
    Fried_Queen_Koi_plus,
    Sculpin_Gratin_plus,
    Grilled_Wolverine_and_Herbs_plus,
    Albinea_Juice,
    Grilled_Chicken_and_Herbs,
    Roast_Duck,
    Grilled_Chicken_and_Herbs_plus,
    Roast_Duck_plus,
    Sacred_Galewind_Shoes,
    Sacred_Floral_Robe,
    Sacred_Snowmelt_Drop,
    Sacred_Moonstone,
    Item55,
    Item56,
    Item57,
    Item58,
    Item59,
    Item60,
    Item61,
    Item62,
    Item63,
    Item64,
    Item65,
    Item66,
    Item67,
    Item68,
    Item69,
    Item70,
    Item71,
    Item72,
    Item73,
    Item74,
    Item75,
    Item76,
    Item77,
    Item78,
    Item79,
    Item80,
    Item81,
    Item82,
    Item83,
    Item84,
    Item85,
    Item86,
    Item87,
    Item88,
    Item89,
    Item90,
    Item91,
    Item92,
    Item93,
    Item94,
    Item95,
    Item96,
    Item97,
    Item98,
    Item99,
    Item100,
    Item101,
    Item102,
    Item103,
    Item104,
    Item105,
    Item106,
    Item107,
    Item108,
    Item109,
    Item110,
    Item111,
    Item112,
    Item113,
    Item114,
    Item115,
    Item116,
    Item117,
    Item118,
    Item119,
    Item120,
    Item121,
    Item122,
    Item123,
    Item124,
    Item125,
    Thorn_Dragon_Sign = 126,
    Wind_Dragon_Sign = 127,
    Sky_Dragon_Sign,
    Crusher_Dragon_Sign,
    Shield_Dragon_Sign,
    Bloom_Dragon_Sign,
    Light_Dragon_Sign,
    Flame_Dragon_Sign,
    Grim_Dragon_Sign,
    Craft_Dragon_Sign,
    Kalpa_Dragon_Sign,
    Earth_Dragon_Sign,
    Ice_Dragon_Sign,
    Fissure_Dragon_Sign,
    Water_Dragon_Sign,
    Storm_Dragon_Sign,
    Lightning_Dragon_Sign,
    Dark_Dragon_Sign,
    Star_Dragon_Sign,
    Snow_Dragon_Sign,
    Aegis_Dragon_Sign,
    Crest_Stone,
    Rocky_Burdock,
    Premiun_Magic_Herbs,
    Ailell_Pomegranate,
    Speed_Carrot,
    Miracle_Bean,
    Ambrosia,
    White_Verona,
    Golden_Apple,
    Fruit_of_Life,
    Dark_Seal = 157,
    Beginner_Seal = 158,
    Item159,
    Item160,
    Item161,
    Item162,
    Item163,
    Item164,
    Item165,
    Item166,
    Item167,
    Item168,
    Item169,
    Item170,
    Item171,
    Item172,
    Item173,
    Item174,
    Item175,
    Item176,
    Item177,
    Item178,
    Item179,
    Item180,
    Item181,
    Item182,
    Item183,
    Item184,
    Item185,
    Item186,
    Item187,
    Item188,
    Item189,
    Item190,
    Item191,
    Item192,
    Item193,
    Item194,
    Item195,
    Item196,
    Item197,
    Item198,
    Item199,
}

binread_enum!(ItemID, u8);
