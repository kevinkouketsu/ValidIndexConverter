use crate::{Bone, Format};
use std::io::{Read, Write};

#[derive(Default)]
pub struct JsonData;
impl Format<Bone> for JsonData {
    fn read<R: Read>(reader: &mut R) -> Result<Bone, std::io::Error>
    where
        Self: Sized,
    {
        Ok(serde_json::from_reader(reader)?)
    }

    fn write<W: Write>(writer: &mut W, ani_list: Bone) -> Result<(), std::io::Error> {
        writer.write_all(serde_json::to_string_pretty(&ani_list)?.as_bytes())
    }
}
