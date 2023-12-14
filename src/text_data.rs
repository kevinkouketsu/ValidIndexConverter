use crate::{Bone, Format, MAX_VALID_ANI_LIST, SIZE_OF_VALID_ANI_LIST};
use std::io::{BufRead, BufReader, Read, Write};

#[derive(Default)]
pub struct TextData;
impl Format<Bone> for TextData {
    fn read<R: Read>(reader: &mut R) -> Result<Bone, std::io::Error>
    where
        Self: Sized,
    {
        let mut ani_list = Bone::default();
        let mut current_key = 0;
        for line in BufReader::new(reader).lines() {
            let line = line?;
            if line.starts_with("BONE=") {
                let index = line
                    .split_once('=')
                    .expect("BONE has to have = separator")
                    .1;

                current_key = index.parse().map_err(|_| {
                    std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "The bone index must be a numeric",
                    )
                })?;
            } else {
                let parts = line.split_once('=').unwrap();
                let key: usize = parts.0.parse().map_err(|_| {
                    std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "The bone index must be a numeric",
                    )
                })?;

                let value = parts.1.parse().unwrap_or(0);
                ani_list.ani[current_key].index[key] = value;
            }
        }

        Ok(ani_list)
    }

    fn write<W: Write>(writer: &mut W, ani_list: Bone) -> Result<(), std::io::Error> {
        for bone_index in 0..MAX_VALID_ANI_LIST {
            writeln!(writer, "BONE={}", bone_index)?;

            for index in 0..SIZE_OF_VALID_ANI_LIST {
                if ani_list.ani[bone_index].index[index] == 0 {
                    continue;
                }

                writeln!(
                    writer,
                    "{}={}",
                    index, ani_list.ani[bone_index].index[index]
                )?;
            }
        }
        Ok(())
    }
}
