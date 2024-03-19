use std::io;
use std::io::Read;

pub const MESSAGE_SIZE: usize = std::mem::size_of::<u64>();

pub fn read_number(connection: &mut impl Read) -> io::Result<u64> {
    let mut buf = [0; MESSAGE_SIZE];
    connection.read_exact(&mut buf)?;

    Ok(u64::from_be_bytes(buf))
}
