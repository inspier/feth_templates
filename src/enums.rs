#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

#[macro_use]
macro_rules! binread_enum {
    ($type:ident, $repr:ident) => {
        impl binread::BinRead for $type {
            type Args = ();
            fn read_options<R: binread::io::Read + binread::io::Seek>(
                reader: &mut R,
                options: &binread::ReadOptions,
                args: Self::Args,
            ) -> binread::BinResult<Self> {
                let byte = $repr::read_options(reader, options, args)?;
                Ok($type::try_from(byte).expect("Unknown value provided."))
            }
        }
    };
}

pub mod ability_id;
pub mod allegiance;
pub mod battalions;
pub mod bgm;
pub mod character_id;
pub mod character_stats;
pub mod class_name;
pub mod crests;
pub mod equipment_id;
pub mod gender_flag;
pub mod general_merchant_items;
pub mod gift_id;
pub mod item_id;
pub mod level;
pub mod merchant_type;
pub mod misc_item;
pub mod names;
pub mod palette_id;
pub mod ranks;
pub mod shop_availability;
pub mod spell_id;
pub mod truefalse;
pub mod weapon_id;
