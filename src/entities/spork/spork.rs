use crate::entities::spork::spork_hash::SporkHash;

#[derive(Debug)]
pub struct Spork<'a> {
    pub id: i32,
    pub signature: &'a [u8],
    pub time_signed: i64,
    pub value: i64,

    pub spork_hash: SporkHash<'a>,
}
