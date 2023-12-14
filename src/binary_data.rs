use deku::prelude::*;
use std::io::{Read, Write};

use crate::{Bone, Format};

#[derive(Default)]
pub struct BinaryData;
impl Format<Bone> for BinaryData {
    fn read<R: Read>(reader: &mut R) -> Result<Bone, std::io::Error>
    where
        Self: Sized,
    {
        let mut data = Vec::new();
        reader.read_to_end(&mut data)?;

        let (_, data) = Bone::from_bytes((&data, 0))?;
        Ok(data)
    }

    fn write<W: Write>(writer: &mut W, list: Bone) -> Result<(), std::io::Error> {
        writer.write_all(&list.to_bytes()?)
    }
}
