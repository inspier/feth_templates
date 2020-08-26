use deku::prelude::*;

#[derive(Debug, DekuRead)]
#[deku(id_type = "u8")]
pub enum MiscItem {
    #[deku(id = "0")]
    Insect_Larva,
    #[deku(id = "1")]
    Minnow_Fillet,
    #[deku(id = "2")]
    Fresh_Insect_Larva,
    #[deku(id = "3")]
    Fresh_Minnow_Fillet,
    #[deku(id = "4")]
    Bristle_Worm,
    #[deku(id = "5")]
    Pond_Snail,
    #[deku(id = "6")]
    Fresh_Bristle_Worm,
    #[deku(id = "7")]
    Premiun_Pond_Snail,
    #[deku(id = "8")]
    Shrimp,
    #[deku(id = "9")]
    Fiddler_Crab,
    #[deku(id = "10")]
    Earthworm,
    #[deku(id = "11")]
    Premium_Shrimp,
    #[deku(id = "12")]
    Premium_Fiddler_Crab,
    #[deku(id = "13")]
    Premium_Earthworm,
    #[deku(id = "14")]
    Pupa,
    #[deku(id = "15")]
    Grub,
    #[deku(id = "16")]
    Beetle,
    #[deku(id = "17")]
    Blowfly,
    #[deku(id = "18")]
    Premium_Pupa,
    #[deku(id = "19")]
    Premium_Grub,
    #[deku(id = "20")]
    Premium_Beetle,
    #[deku(id = "21")]
    Premium_Blowfly,
    #[deku(id = "22")]
    Squid_Ball,
    #[deku(id = "23")]
    Herring_Bait,
    #[deku(id = "24")]
    Bait_24,
    #[deku(id = "25")]
    Bait_25,
    #[deku(id = "26")]
    Bait_26,
    #[deku(id = "27")]
    Bait_27,
    #[deku(id = "28")]
    Bait_28,
    #[deku(id = "29")]
    Bait_29,
    #[deku(id = "30")]
    Tournmant_Bait,
    #[deku(id = "31")]
    Flayns_Bait,
    #[deku(id = "32")]
    Mixed_Herb_Seeds,
    #[deku(id = "33")]
    Western_Foldlan_Seeds,
    #[deku(id = "34")]
    Root_Vegetable_Seeds,
    #[deku(id = "35")]
    Magical_Herb_Seeds,
    #[deku(id = "36")]
    Noah_Fruit_Seeds,
    #[deku(id = "37")]
    Albinean_Nut_Seeds,
    #[deku(id = "38")]
    Vegetable_Seeds,
    #[deku(id = "39")]
    Northern_Foldlan_Seeds,
    #[deku(id = "40")]
    Verona_Seeds,
    #[deku(id = "41")]
    Morfis_Plum_Seeds,
    #[deku(id = "42")]
    Southern_Fodlan_Seeds,
    #[deku(id = "43")]
    Morfis_Seeds,
    #[deku(id = "44")]
    Nordsalat_Seeds,
    #[deku(id = "45")]
    Boa_Fruit_Seeds,
    #[deku(id = "46")]
    Albinean_Seeds,
    #[deku(id = "47")]
    Eastern_Fodlan_Seeds,
    #[deku(id = "48")]
    Magdred_Kirsch_Seeds,
    #[deku(id = "49")]
    Angelica_Seeds,
    #[deku(id = "50")]
    Mixed_Fruit_Seeds,
    #[deku(id = "51")]
    Albinean_Berry_Seeds,
    #[deku(id = "52")]
    Red_Flower_Seeds,
    #[deku(id = "53")]
    White_Flowers_Seeds,
    #[deku(id = "54")]
    Blue_Flowers_Seeds,
    #[deku(id = "55")]
    Purple_Flowers_Seeds,
    #[deku(id = "56")]
    Yellow_Flowers_Seeds,
    #[deku(id = "57")]
    Green_Flowers_Seeds,
    #[deku(id = "58")]
    Pale_Blue_Flower_Seeds,
    #[deku(id = "59")]
    Seeds_27,
    #[deku(id = "60")]
    Seeds_28,
    #[deku(id = "61")]
    Seeds_29,
    #[deku(id = "62")]
    Seeds_30,
    #[deku(id = "63")]
    Dedues_Seeds,
    #[deku(id = "64")]
    Smithing_Stone,
    #[deku(id = "65")]
    Black_Sand_Steel,
    #[deku(id = "66")]
    Wootz_Steel,
    #[deku(id = "67")]
    Arcane_Crystal,
    #[deku(id = "68")]
    Mythril,
    #[deku(id = "69")]
    Umbral_Steel,
    #[deku(id = "70")]
    Agarthuim,
    #[deku(id = "71")]
    Venomstone,
    #[deku(id = "72")]
    Ore_08,
    #[deku(id = "73")]
    Ore_09,
    #[deku(id = "74")]
    Ore_10,
    #[deku(id = "75")]
    Ore_11,
    #[deku(id = "76")]
    Ore_12,
    #[deku(id = "77")]
    Ore_13,
    #[deku(id = "78")]
    Ore_14,
    #[deku(id = "79")]
    Ore_15,
    #[deku(id = "80")]
    Ore_16,
    #[deku(id = "81")]
    Ore_17,
    #[deku(id = "82")]
    Ore_18,
    #[deku(id = "83")]
    Ore_19,
    #[deku(id = "84")]
    Ore_20,
    #[deku(id = "85")]
    Ore_21,
    #[deku(id = "86")]
    Ore_22,
    #[deku(id = "87")]
    Ore_23,
    #[deku(id = "88")]
    Ore_24,
    #[deku(id = "89")]
    Ore_25,
    #[deku(id = "90")]
    Ore_26,
    #[deku(id = "91")]
    Ore_27,
    #[deku(id = "92")]
    Ore_28,
    #[deku(id = "93")]
    Ore_29,
    #[deku(id = "94")]
    Ore_30,
    #[deku(id = "95")]
    Tutorial_Ore,
    #[deku(id = "96")]
    Airmid_Goby,
    #[deku(id = "97")]
    Caledonian_Crayfish,
    #[deku(id = "98")]
    White_Trout,
    #[deku(id = "99")]
    Teutates_loach,
    #[deku(id = "100")]
    Airmid_Pike,
    #[deku(id = "101")]
    Caledonian_Gar,
    #[deku(id = "102")]
    Queen_Loach,
    #[deku(id = "103")]
    Ancient_Gar,
    #[deku(id = "104")]
    Teutates_Pike,
    #[deku(id = "105")]
    Teutates_Shrimp,
    #[deku(id = "106")]
    Bullhead,
    #[deku(id = "107")]
    Albinean_Herring,
    #[deku(id = "108")]
    Golden_Fish,
    #[deku(id = "109")]
    Platinum_Fish,
    #[deku(id = "110")]
    Fodlandy,
    #[deku(id = "111")]
    Goddess_Messenger,
    #[deku(id = "112")]
    Silverfish,
    #[deku(id = "113")]
    Fish_17,
    #[deku(id = "114")]
    Fish_18,
    #[deku(id = "115")]
    Fish_19,
    #[deku(id = "116")]
    Fish_20,
    #[deku(id = "117")]
    Fish_21,
    #[deku(id = "118")]
    Fish_22,
    #[deku(id = "119")]
    Fish_23,
    #[deku(id = "120")]
    Fish_24,
    #[deku(id = "121")]
    Fish_25,
    #[deku(id = "122")]
    Fish_26,
    #[deku(id = "123")]
    Fish_27,
    #[deku(id = "124")]
    Fish_28,
    #[deku(id = "125")]
    Fish_29,
    #[deku(id = "126")]
    Fish_30,
    #[deku(id = "127")]
    Carassius,
    #[deku(id = "128")]
    Corn,
    #[deku(id = "129")]
    Apple,
    #[deku(id = "130")]
    Sugar_Cane,
    #[deku(id = "131")]
    Barley,
    #[deku(id = "132")]
    Weeds,
    #[deku(id = "133")]
    Zanado_Fruit,
    #[deku(id = "134")]
    Dried_Vegetables,
    #[deku(id = "135")]
    Ailell_Grass,
    #[deku(id = "136")]
    Zanado_Treasure_Fruit,
    #[deku(id = "137")]
    Tomato,
    #[deku(id = "138")]
    Noa_Fruit,
    #[deku(id = "139")]
    Peach_Currant,
    #[deku(id = "140")]
    Verona,
    #[deku(id = "141")]
    Morfis_Plum,
    #[deku(id = "142")]
    Nordsalat,
    #[deku(id = "143")]
    Boa_Fruit,
    #[deku(id = "144")]
    Magdred_Kirsch,
    #[deku(id = "145")]
    Angelica,
    #[deku(id = "146")]
    Albinean_Berries,
    #[deku(id = "147")]
    Onion,
    #[deku(id = "148")]
    Carrot,
    #[deku(id = "149")]
    Turnip,
    #[deku(id = "150")]
    Chickpeas,
    #[deku(id = "151")]
    Cabbage,
    #[deku(id = "152")]
    Crops_24,
    #[deku(id = "153")]
    Crops_25,
    #[deku(id = "154")]
    Crops_26,
    #[deku(id = "155")]
    Crops_27,
    #[deku(id = "156")]
    Crops_28,
    #[deku(id = "157")]
    Crops_29,
    #[deku(id = "158")]
    Crops_30,
    #[deku(id = "159")]
    Starter_Vegetable,
    #[deku(id = "160")]
    Poultry,
    #[deku(id = "161")]
    Rabbit,
    #[deku(id = "162")]
    Foldlan_Pheasant,
    #[deku(id = "163")]
    Gronder_Fox,
    #[deku(id = "164")]
    Wild_Game,
    #[deku(id = "165")]
    Boar,
    #[deku(id = "166")]
    Grouse,
    #[deku(id = "167")]
    Duscur_Bear,
    #[deku(id = "168")]
    Oghma_Wolverine,
    #[deku(id = "169")]
    Albinean_Moose,
    #[deku(id = "170")]
    Rabbit2,
    #[deku(id = "171")]
    Gronder_Fox2,
    #[deku(id = "172")]
    Foldlan_Pheasant2,
    #[deku(id = "173")]
    Meat_13,
    #[deku(id = "174")]
    Meat_14,
    #[deku(id = "175")]
    Meat_15,
    #[deku(id = "176")]
    Meat_16,
    #[deku(id = "177")]
    Meat_17,
    #[deku(id = "178")]
    Meat_18,
    #[deku(id = "179")]
    Meat_19,
    #[deku(id = "180")]
    Meat_20,
    #[deku(id = "181")]
    Meat_21,
    #[deku(id = "182")]
    Meat_22,
    #[deku(id = "183")]
    Meat_23,
    #[deku(id = "184")]
    Meat_24,
    #[deku(id = "185")]
    Meat_25,
    #[deku(id = "186")]
    Meat_26,
    #[deku(id = "187")]
    Meat_27,
    #[deku(id = "188")]
    Meat_28,
    #[deku(id = "189")]
    Meat_29,
    #[deku(id = "190")]
    Meat_30,
    #[deku(id = "191")]
    Starter_Meat,
    #[deku(id = "192")]
    Bergamot,
    #[deku(id = "193")]
    Sweet_Apple_Blend,
    #[deku(id = "194")]
    Almyran_Pine_Needles,
    #[deku(id = "195")]
    Albinean_Berry_Blend,
    #[deku(id = "196")]
    Southern_Fruit_Blend,
    #[deku(id = "197")]
    Rose_Petal_Blend,
    #[deku(id = "198")]
    Mint_Leaves,
    #[deku(id = "199")]
    Crescent_Moon_Tea,
    #[deku(id = "200")]
    Dagda_Fruit_Blend,
    #[deku(id = "201")]
    Almond_Blend,
    #[deku(id = "202")]
    Honeyed_Fruit_Blend,
    #[deku(id = "203")]
    Cinnamon_Blend,
    #[deku(id = "204")]
    Seiros_Blend,
    #[deku(id = "205")]
    Four_Spice_Blend,
    #[deku(id = "206")]
    Ginger_Tea,
    #[deku(id = "207")]
    Angelica_Tea,
    #[deku(id = "208")]
    Lanender_Blend,
    #[deku(id = "209")]
    Chamomile,
    #[deku(id = "210")]
    Hresvelg_Blend,
    #[deku(id = "211")]
    Lecicester_Cortania,
    #[deku(id = "212")]
    Tea_of_the_Saints,
    #[deku(id = "213")]
    Tea_Leaves_22,
    #[deku(id = "214")]
    Tea_Leaves_23,
    #[deku(id = "215")]
    Tea_Leaves_24,
    #[deku(id = "216")]
    Tea_Leaves_25,
    #[deku(id = "217")]
    Tea_Leaves_26,
    #[deku(id = "218")]
    Tea_Leaves_27,
    #[deku(id = "219")]
    Tea_Leaves_28,
    #[deku(id = "220")]
    Tea_Leaves_29,
    #[deku(id = "221")]
    Tea_Leaves_30,
    #[deku(id = "222")]
    Tea_Leaves_31,
    #[deku(id = "255")]
    No_Misc_Item,
}
