#[derive(Debug)]
#[repr(u8)]
pub enum CharacterStat {
    Str,
    Mag,
    Dex,
    Spd,
    Lck,
    Def,
    Res,
    Mov,
    Cha,
    NullStat,
}
