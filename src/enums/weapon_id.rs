use deku::prelude::*;

#[derive(Debug, DekuRead)]
#[deku(id_type = "u16", id_bytes = "2")]
pub enum WeaponID {
    #[deku(id = "0")]
    Unarmed,
    #[deku(id = "1")]
    Broken_Sword,
    #[deku(id = "2")]
    Broken_Lance,
    #[deku(id = "3")]
    Broken_Axe,
    #[deku(id = "4")]
    Broken_Bow,
    #[deku(id = "5")]
    Broken_Gauntlet,
    #[deku(id = "6")]
    Iron_Sword,
    #[deku(id = "7")]
    Steel_Sword,
    #[deku(id = "8")]
    Silver_Sword,
    #[deku(id = "9")]
    Brave_Sword,
    #[deku(id = "10")]
    Killing_Edge,
    #[deku(id = "11")]
    Training_Sword,
    #[deku(id = "12")]
    Iron_Lance,
    #[deku(id = "13")]
    Steel_Lance,
    #[deku(id = "14")]
    Silver_Lance,
    #[deku(id = "15")]
    Brave_Lance,
    #[deku(id = "16")]
    Killer_Lance,
    #[deku(id = "17")]
    Training_Lance,
    #[deku(id = "18")]
    Iron_Axe,
    #[deku(id = "19")]
    Steel_Axe,
    #[deku(id = "20")]
    Silver_Axe,
    #[deku(id = "21")]
    Brave_Axe,
    #[deku(id = "22")]
    Killer_Axe,
    #[deku(id = "23")]
    Training_Axe,
    #[deku(id = "24")]
    Iron_Bow,
    #[deku(id = "25")]
    Steel_Bow,
    #[deku(id = "26")]
    Silver_Bow,
    #[deku(id = "27")]
    Brave_Bow,
    #[deku(id = "28")]
    Killer_Bow,
    #[deku(id = "29")]
    Training_Bow,
    #[deku(id = "30")]
    Iron_Gauntlets,
    #[deku(id = "31")]
    Steel_Gauntlets,
    #[deku(id = "32")]
    Silver_Gauntlets,
    #[deku(id = "33")]
    Training_Gauntlets,
    #[deku(id = "34")]
    Levin_Sword,
    #[deku(id = "35")]
    Bolt_Axe,
    #[deku(id = "36")]
    Magic_Bow,
    #[deku(id = "37")]
    Javalin,
    #[deku(id = "38")]
    Short_Spear,
    #[deku(id = "39")]
    Spear,
    #[deku(id = "40")]
    Hand_Axe,
    #[deku(id = "41")]
    Short_Axe,
    #[deku(id = "42")]
    Tomahawk,
    #[deku(id = "43")]
    Longbow,
    #[deku(id = "44")]
    Mini_Bow,
    #[deku(id = "45")]
    Armorslayer,
    #[deku(id = "46")]
    Rapier,
    #[deku(id = "47")]
    Horseslayer,
    #[deku(id = "48")]
    Hammer,
    #[deku(id = "49")]
    Blessed_Lance,
    #[deku(id = "50")]
    Blessed_Bow,
    #[deku(id = "51")]
    Devil_Sword,
    #[deku(id = "52")]
    Devil_Axe,
    #[deku(id = "53")]
    Wo_Dao,
    #[deku(id = "54")]
    Crescent_Sickle,
    #[deku(id = "55")]
    Sword_of_Seiros,
    #[deku(id = "56")]
    Sword_of_Begalta,
    #[deku(id = "57")]
    Sword_of_Moralta,
    #[deku(id = "58")]
    Cursed_Ashiya_Sword,
    #[deku(id = "59")]
    Sword_or_Zoltan,
    #[deku(id = "60")]
    Thunderbrand,
    #[deku(id = "61")]
    Blutgang,
    #[deku(id = "62")]
    Sword_of_the_Creator,
    #[deku(id = "63")]
    Lance_of_Zoltan,
    #[deku(id = "64")]
    Lance_of_Ruin,
    #[deku(id = "65")]
    Areadbhar,
    #[deku(id = "66")]
    Luin,
    #[deku(id = "67")]
    Spear_of_Assal,
    #[deku(id = "68")]
    Scythe_of_Sariel,
    #[deku(id = "69")]
    Arrow_of_Indra,
    #[deku(id = "70")]
    Freikugel,
    #[deku(id = "71")]
    Crusher,
    #[deku(id = "72")]
    Axe_of_Ukonvasara,
    #[deku(id = "73")]
    Axe_of_Zoltan,
    #[deku(id = "74")]
    Tathlum_Bow,
    #[deku(id = "75")]
    The_Inexhaustiable,
    #[deku(id = "76")]
    Bow_of_Zoltan,
    #[deku(id = "77")]
    Failnaught,
    #[deku(id = "78")]
    Dragon_Claw,
    #[deku(id = "79")]
    Mace,
    #[deku(id = "80")]
    Athame,
    #[deku(id = "81")]
    Ridill,
    #[deku(id = "82")]
    Aymr,
    #[deku(id = "83")]
    Dark_Sword_of_The_Creator,
    #[deku(id = "84")]
    Venin_Edge,
    #[deku(id = "85")]
    Venin_Lance,
    #[deku(id = "86")]
    Venin_Axe,
    #[deku(id = "87")]
    Vinen_Bow,
    #[deku(id = "88")]
    Mercurius,
    #[deku(id = "89")]
    Gradivus,
    #[deku(id = "90")]
    Hauteclere,
    #[deku(id = "91")]
    Parthia,
    #[deku(id = "92")]
    Killer_Knuckles,
    #[deku(id = "93")]
    Aura_Knuckles,
    #[deku(id = "94")]
    Rusted_Sword_Iron,
    #[deku(id = "95")]
    Rusted_Sword_Steel,
    #[deku(id = "96")]
    Rusted_Sword_Silver,
    #[deku(id = "97")]
    Rusted_Sword_Brave,
    #[deku(id = "98")]
    Rusted_Sword_Mercurius,
    #[deku(id = "99")]
    Rusted_Lance_Iron,
    #[deku(id = "100")]
    Rusted_Lance_Steel,
    #[deku(id = "101")]
    Rusted_Lance_Silver,
    #[deku(id = "102")]
    Rusted_Lance_Brave,
    #[deku(id = "103")]
    Rusted_Lance_Gradivus,
    #[deku(id = "104")]
    Rusted_Axe_Iron,
    #[deku(id = "105")]
    Rusted_Axe_Steel,
    #[deku(id = "106")]
    Rusted_Axe_Silver,
    #[deku(id = "107")]
    Rusted_Axe_Brave,
    #[deku(id = "108")]
    Rusted_Axe_Hauteclere,
    #[deku(id = "109")]
    Rusted_Bow_Iron,
    #[deku(id = "110")]
    Rusted_Bow_Steel,
    #[deku(id = "111")]
    Rusted_Bow_Silver,
    #[deku(id = "112")]
    Rusted_Bow_Brave,
    #[deku(id = "113")]
    Rusted_Bow_Parthia,
    #[deku(id = "114")]
    Rusted_Gauntlets_Iron,
    #[deku(id = "115")]
    Rusted_Gauntlets_Steel,
    #[deku(id = "116")]
    Rusted_Gauntlets_Silver,
    #[deku(id = "117")]
    Rusted_Gauntlets_Dragon_Claws,
    #[deku(id = "118")]
    Iron_Sword_Plus,
    #[deku(id = "119")]
    Steel_Sword_Plus,
    #[deku(id = "120")]
    Silver_Sword_Plus,
    #[deku(id = "121")]
    Brave_Sword_Plus,
    #[deku(id = "122")]
    Killing_Edge_Plus,
    #[deku(id = "123")]
    Training_Sword_Plus,
    #[deku(id = "124")]
    Iron_Lance_Plus,
    #[deku(id = "125")]
    Steel_Lance_Plus,
    #[deku(id = "126")]
    Silver_Lance_Plus,
    #[deku(id = "127")]
    Brave_Lance_Plus,
    #[deku(id = "128")]
    Killer_Lance_Plus,
    #[deku(id = "129")]
    Training_Lance_Plus,
    #[deku(id = "130")]
    Iron_Axe_Plus,
    #[deku(id = "131")]
    Steel_Axe_Plus,
    #[deku(id = "132")]
    Silver_Axe_Plus,
    #[deku(id = "133")]
    Brave_Axe_Plus,
    #[deku(id = "134")]
    Killer_Axe_Plus,
    #[deku(id = "135")]
    Training_Axe_Plus,
    #[deku(id = "136")]
    Iron_Bow_Plus,
    #[deku(id = "137")]
    Steel_Bow_Plus,
    #[deku(id = "138")]
    Silver_Bow_Plus,
    #[deku(id = "139")]
    Brave_Bow_Plus,
    #[deku(id = "140")]
    Killer_Bow_Plus,
    #[deku(id = "141")]
    Training_Bow_Plus,
    #[deku(id = "142")]
    Iron_Gauntlets_Plus,
    #[deku(id = "143")]
    Steel_Gauntlets_Plus,
    #[deku(id = "144")]
    Silver_Gauntlets_Plus,
    #[deku(id = "145")]
    Training_Gauntlets_Plus,
    #[deku(id = "146")]
    Levin_Sword_Plus,
    #[deku(id = "147")]
    Bolt_Axe_Plus,
    #[deku(id = "148")]
    Magic_Bow_Plus,
    #[deku(id = "149")]
    Javalin_Plus,
    #[deku(id = "150")]
    Short_Spear_Plus,
    #[deku(id = "151")]
    Spear_Plus,
    #[deku(id = "152")]
    Hand_Axe_Plus,
    #[deku(id = "153")]
    Short_Axe_Plus,
    #[deku(id = "154")]
    Tomahawk_Plus,
    #[deku(id = "155")]
    Longbow_Plus,
    #[deku(id = "156")]
    Mini_Bow_Plus,
    #[deku(id = "157")]
    Armorslayer_Plus,
    #[deku(id = "158")]
    Rapier_Plus,
    #[deku(id = "159")]
    Horseslayer_Plus,
    #[deku(id = "160")]
    Hammer_Plus,
    #[deku(id = "161")]
    Blessed_Lance_Plus,
    #[deku(id = "162")]
    Blessed_Bow_Plus,
    #[deku(id = "163")]
    Devil_Sword_Plus,
    #[deku(id = "164")]
    Devil_Axe_Plus,
    #[deku(id = "165")]
    Wo_Dao_Plus,
    #[deku(id = "166")]
    Crescent_Sickle_Plus,
    #[deku(id = "167")]
    Cursed_Ashiya_Sword_Plus,
    #[deku(id = "168")]
    Sword_or_Zoltan_Plus,
    #[deku(id = "169")]
    Lance_of_Zoltan_Plus,
    #[deku(id = "170")]
    Arrow_of_Indra_Plus,
    #[deku(id = "171")]
    Axe_of_Zoltan_Plus,
    #[deku(id = "172")]
    Bow_of_Zoltan_Plus,
    #[deku(id = "173")]
    Dragon_Claw_Plus,
    #[deku(id = "174")]
    Mace_Plus,
    #[deku(id = "175")]
    Venin_Edge_Plus,
    #[deku(id = "176")]
    Venin_Lance_Plus,
    #[deku(id = "177")]
    Venin_Axe_Plus,
    #[deku(id = "178")]
    Vinen_Bow_Plus,
    #[deku(id = "179")]
    Killer_Knuckles_Plus,
    #[deku(id = "180")]
    Aura_Knuckles_Plus,
    #[deku(id = "181")]
    Sublime_Sword_of_the_Creator,
    #[deku(id = "182")]
    Dark_Thunderbrand,
    #[deku(id = "183")]
    Dark_Blutgang,
    #[deku(id = "184")]
    Dark_Lance_of_Ruin,
    #[deku(id = "185")]
    Dark_Areadbhar,
    #[deku(id = "186")]
    Dark_Luin,
    #[deku(id = "187")]
    Dark_Freikugel,
    #[deku(id = "188")]
    Dark_Crusher,
    #[deku(id = "189")]
    Dark_Failnaught,
    #[deku(id = "190")]
    Weapon190,
    #[deku(id = "191")]
    Weapon191,
    #[deku(id = "192")]
    Weapon192,
    #[deku(id = "193")]
    Weapon193,
    #[deku(id = "194")]
    Weapon194,
    #[deku(id = "195")]
    Weapon195,
    #[deku(id = "196")]
    Weapon196,
    #[deku(id = "197")]
    Weapon197,
    #[deku(id = "198")]
    Weapon198,
    #[deku(id = "199")]
    Weapon199,
}
