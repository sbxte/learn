#[derive(Debug, PartialEq, Eq)]
pub struct ColorType {
    pub grayscale: bool,
    pub palette: bool,
    pub color: bool,
    pub alpha: bool,
}

impl ColorType {
    pub fn from_byte(byte: u8) -> Self {
        let palette = byte & 1 != 0;
        let color = byte & 2 != 0;
        let alpha = byte & 4 != 0;
        let grayscale = !(palette || color || alpha);
        Self {
            grayscale,
            palette,
            color,
            alpha,
        }
    }
}

impl Default for ColorType {
    fn default() -> Self {
        Self {
            grayscale: true,
            palette: false,
            color: false,
            alpha: false,
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Pixel {
    pub r: u16,
    pub g: u16,
    pub b: u16,
    pub a: u16,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Png {
    pub width: u32,
    pub height: u32,
    pub bit_depth: u8,
    pub color_type: ColorType,
    pub compression_method: u8,
    pub filter_method: u8,
    pub interlace_method: u8,
    pub palette: Vec<u8>,
    pub pixels: Vec<Pixel>,
}

impl Png {
    pub fn empty() -> Self {
        Default::default()
    }

    pub fn new(bytes: &[u8]) -> Self {
        let mut construct = Self::empty();

        // 8-byte header
        let _ = bytes[..8];
        let bytes = &bytes[8..];

        // Chunks
        let mut i = 0;
        let mut pixel_data = vec![];
        while i < bytes.len() {
            // Chunk headers
            let clength: u32 = unsafe {
                std::mem::transmute::<[u8; 4], u32>(bytes[i..i + 4].try_into().unwrap()).to_be()
            };

            let ctype: [u8; 4] = bytes[i + 4..i + 8].try_into().unwrap();
            let ctype = ctype.map(|f| f as char);

            let cdata = &bytes[i + 8..i + 8 + clength as usize];
            let _crc = &bytes[i + 8 + clength as usize..i + 8 + clength as usize + 4];

            if PngChunk::parse_chunk(ctype, cdata, &mut construct, &mut pixel_data) {
                break;
            }

            i += 8 + clength as usize + 4;
        }

        Self::parse_idat(&pixel_data, &mut construct);

        construct
    }

    fn parse_idat(bytes: &[u8], construct: &mut Self) {
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
                    Pixel {
                        r: bytes[bptr] as u16,
                        g: bytes[bptr + 1] as u16,
                        b: bytes[bptr + 2] as u16,
                        a: bytes[bptr + 3] as u16,
                    }
                } else {
                    Pixel {
                        r: unsafe {
                            std::mem::transmute::<[u8; 2], u16>(
                                bytes[bptr..bptr + 2].try_into().unwrap(),
                            )
                        },
                        g: unsafe {
                            std::mem::transmute::<[u8; 2], u16>(
                                bytes[bptr + 2..bptr + 4].try_into().unwrap(),
                            )
                        },
                        b: unsafe {
                            std::mem::transmute::<[u8; 2], u16>(
                                bytes[bptr + 4..bptr + 6].try_into().unwrap(),
                            )
                        },
                        a: unsafe {
                            std::mem::transmute::<[u8; 2], u16>(
                                bytes[bptr + 6..bptr + 8].try_into().unwrap(),
                            )
                        },
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
}

struct PngChunk;

impl PngChunk {
    /// Returns true when chunk parsing loop should end
    fn parse_chunk(
        ctype: [char; 4],
        bytes: &[u8],
        construct: &mut Png,
        pixel_data: &mut Vec<u8>,
    ) -> bool {
        match ctype {
            ['I', 'H', 'D', 'R'] => Self::parse_chunk_ihdr(bytes, construct),
            ['P', 'L', 'T', 'E'] => Self::parse_chunk_plte(bytes, construct),
            ['I', 'D', 'A', 'T'] => Self::parse_chunk_idat(bytes, construct, pixel_data),
            ['I', 'E', 'N', 'D'] => return true,
            _ => {
                dbg!(&ctype);
            }
        };
        false
    }

    fn parse_chunk_ihdr(bytes: &[u8], construct: &mut Png) {
        construct.width =
            unsafe { std::mem::transmute::<[u8; 4], u32>(bytes[0..4].try_into().unwrap()) }.to_be();
        construct.height =
            unsafe { std::mem::transmute::<[u8; 4], u32>(bytes[4..8].try_into().unwrap()) }.to_be();
        construct.bit_depth = bytes[8];
        construct.color_type = ColorType::from_byte(bytes[9]);
        construct.compression_method = bytes[10];
        construct.filter_method = bytes[11];
        construct.interlace_method = bytes[12];
    }

    fn parse_chunk_plte(bytes: &[u8], construct: &mut Png) {
        construct.palette.copy_from_slice(bytes);
    }

    fn parse_chunk_idat(bytes: &[u8], construct: &mut Png, pixel_data: &mut Vec<u8>) {
        // TODO: Implement filters and compression

        let bptr = if construct.filter_method == 0 { 1 } else { 0 };

        pixel_data.extend(&bytes[bptr..]);
    }
}
