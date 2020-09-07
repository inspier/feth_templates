use num_enum::TryFromPrimitive;
use std::convert::TryFrom;

#[derive(Debug, TryFromPrimitive)]
#[repr(i8)]
/// Enums for minimum ranks
pub enum MinRanks {
    E = 0,
    EPlus = 1,
    D = 2,
    DPlus = 3,
    C = 4,
    CPlus = 5,
    B = 6,
    BPlus = 7,
    A = 8,
    APlus = 9,
    S = 10,
    SPlus = 11,
}

binread_enum!(MinRanks, i8);

#[derive(Debug)]
#[repr(u8)]
pub enum RankStats {
    Sword,
    Lances,
    Axes,
    Bows,
    Brawling,
    Reason,
    Faith,
    Authority,
    Armor,
    Riding,
    Flying,
    NullRequirement,
    No_Stat = 255,
}
