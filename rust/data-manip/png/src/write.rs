use crate::*;

pub const HEADER: [u8; 8] = [0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a];

pub fn write(png: &Png) -> Vec<u8> {
    let mut b = vec![];

    HEADER.iter().for_each(|c| b.push(*c));
    write_ihdr(&mut b, png);

    b
}

pub fn write_chunk(
    buf: &mut Vec<u8>,
    clength: [u8; 4],
    ctype: [u8; 4],
    cdata: &[u8],
    crc: [u8; 4],
) {
    buf.extend_from_slice(&clength);
    buf.extend_from_slice(&ctype);
    buf.extend_from_slice(cdata);
    buf.extend_from_slice(&crc);
}

pub fn calc_crc() {}

pub fn write_ihdr(buf: &mut Vec<u8>, png: &Png) {
    todo!()
}
