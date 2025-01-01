use crate::types::DvdInfo;

pub mod human_readable;

pub trait DvdSerializer {
    fn serialize(dvd_info: DvdInfo) -> Result<String, String>;
}