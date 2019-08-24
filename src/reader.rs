use std::{
    io::{
        self,
        Seek,
        SeekFrom,
        Read,
    },
};

pub fn to_bool<R: Read>(reader: &mut R) -> io::Result<bool> {
    let mut buffer = [0u8; 1];
    reader.read(&mut buffer)?;
    Ok(buffer[0] == 1)
}
