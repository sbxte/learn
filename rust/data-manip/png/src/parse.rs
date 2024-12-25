use crate::*;
use casts::*;

pub fn parse_idat_all(bytes: &[u8], construct: &mut Png) {
    let mut bptr = 0;
    while bptr < bytes.len() {
        // Byte length
        let mut blength = if construct.bit_depth >= 8 {
            // 8 = 2^3
            construct.bit_depth as usize >> 3
        } else {
            1
        };

        // Color types
        let pixel = if construct.color_type.color {
            // RGB
            blength *= 4;

            // TODO: Implement alpha color
            if construct.bit_depth == 8 {
                if bptr + 3 >= bytes.len() {
                    dbg!(&bytes[bptr - 4..]);
                }
                Color {
                    r: bytes[bptr] as u16,
                    g: bytes[bptr + 1] as u16,
                    b: bytes[bptr + 2] as u16,
                    a: bytes[bptr + 3] as u16,
                }
            } else {
                Color {
                    r: cast_2u8_u16(&bytes[bptr..bptr + 2]),
                    g: cast_2u8_u16(&bytes[bptr + 2..bptr + 4]),
                    b: cast_2u8_u16(&bytes[bptr + 4..bptr + 6]),
                    a: cast_2u8_u16(&bytes[bptr + 6..bptr + 8]),
                }
            }
        } else {
            // TODO: Implement other color types
            Default::default()
        };

        construct.pixels.push(pixel);

        bptr += blength;
    }
}
/// Returns true when chunk parsing loop should end
pub fn parse_chunk(
    ctype: [char; 4],
    bytes: &[u8],
    construct: &mut Png,
    pixel_data: &mut Vec<u8>,
) -> bool {
    match ctype {
        ['I', 'H', 'D', 'R'] => parse_ihdr(bytes, construct),
        ['P', 'L', 'T', 'E'] => parse_plte(bytes, construct),
        ['I', 'D', 'A', 'T'] => parse_idat(bytes, construct, pixel_data),
        ['I', 'E', 'N', 'D'] => return true,
        ['b', 'K', 'G', 'D'] => parse_bkgd(bytes, construct),
        ['c', 'H', 'R', 'M'] => parse_chrm(bytes, construct),
        ['p', 'H', 'Y', 's'] => parse_phys(bytes, construct),
        ['t', 'E', 'X', 't'] | ['i', 'T', 'X', 't'] | ['z', 'T', 'X', 't'] => {
            parse_text(bytes, construct)
        }
        _ => {
            dbg!(&ctype);
        }
    };
    false
}

fn parse_ihdr(bytes: &[u8], construct: &mut Png) {
    construct.width = cast_4u8_u32(&bytes[0..4]);
    construct.height = cast_4u8_u32(&bytes[4..8]);
    construct.bit_depth = bytes[8];
    construct.color_type = ColorType::from_byte(bytes[9]);
    construct.compression_method = bytes[10];
    construct.filter_method = bytes[11];
    construct.interlace_method = bytes[12];
}

fn parse_plte(bytes: &[u8], construct: &mut Png) {
    construct.palette.copy_from_slice(bytes);
}

fn parse_idat(bytes: &[u8], construct: &mut Png, pixel_data: &mut Vec<u8>) {
    // TODO: Implement filters and compression

    let bptr = if construct.filter_method == 0 { 1 } else { 0 };

    pixel_data.extend(&bytes[bptr..]);
}

fn parse_bkgd(bytes: &[u8], construct: &mut Png) {
    construct.bg_color = match construct.color_type.to_byte() {
        0 | 4 => Some(Color::grayscale(
            cast_2u8_u16(&bytes[0..2]),
            construct.bit_depth,
        )),
        2 | 6 => Some(Color::rgb(
            cast_2u8_u16(&bytes[0..2]),
            cast_2u8_u16(&bytes[2..4]),
            cast_2u8_u16(&bytes[4..6]),
            construct.bit_depth,
        )),
        _ => None,
    }
}

fn parse_chrm(bytes: &[u8], construct: &mut Png) {
    construct.chromaticities = Some(Chromaticity::new(
        cast_tuple_4u8_u32!(&bytes[0..4], &bytes[4..8]),
        cast_tuple_4u8_u32!(&bytes[8..12], &bytes[12..16]),
        cast_tuple_4u8_u32!(&bytes[16..20], &bytes[20..24]),
        cast_tuple_4u8_u32!(&bytes[24..28], &bytes[28..32]),
    ))
}

fn parse_phys(bytes: &[u8], construct: &mut Png) {
    construct.phys_dim = Some(PhysDim::new(
        cast_4u8_u32(&bytes[0..4]),
        cast_4u8_u32(&bytes[4..8]),
        bytes[8],
    ))
}

fn parse_text(bytes: &[u8], construct: &mut Png) {
    let mut v = Vec::with_capacity(bytes.len());
    for b in bytes {
        v.push(*b);
    }
    construct.texts.push(String::from_utf8(v).unwrap())
}
