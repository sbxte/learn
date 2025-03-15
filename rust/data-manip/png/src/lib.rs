/// Utility module for casting types
pub mod casts;

/// Png parsing module
pub mod parse;

/// Png writing module
pub mod write;

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

    pub fn to_byte(&self) -> u8 {
        self.palette as u8 | (self.color as u8) << 2 | (self.alpha as u8) << 3
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
pub struct Color {
    pub r: u16,
    pub g: u16,
    pub b: u16,
    pub a: u16,
}

impl Color {
    pub fn rgb(r: u16, g: u16, b: u16, bit_depth: u8) -> Self {
        Self {
            r,
            g,
            b,
            a: 1 << (bit_depth as u16),
        }
    }

    pub fn grayscale(value: u16, bit_depth: u8) -> Self {
        Self {
            r: value,
            g: value,
            b: value,
            a: 1 << (bit_depth as u16),
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Chromaticity {
    wpx: u32,
    wpy: u32,
    rx: u32,
    ry: u32,
    gx: u32,
    gy: u32,
    bx: u32,
    by: u32,
}

impl Chromaticity {
    pub fn new(wp: (u32, u32), r: (u32, u32), g: (u32, u32), b: (u32, u32)) -> Self {
        Self {
            wpx: wp.0,
            wpy: wp.1,
            rx: r.0,
            ry: r.1,
            gx: g.0,
            gy: g.1,
            bx: b.0,
            by: b.1,
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct PhysDim {
    x: u32,
    y: u32,
    u: u8,
}

impl PhysDim {
    pub fn new(x: u32, y: u32, u: u8) -> Self {
        Self { x, y, u }
    }
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
    pub pixels: Vec<Color>,

    pub bg_color: Option<Color>,
    pub chromaticities: Option<Chromaticity>,
    pub phys_dim: Option<PhysDim>,
    pub texts: Vec<String>,
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
            let clength: u32 = casts::cast_4u8_u32(&bytes[i..i + 4]);
            let ctype: [u8; 4] = bytes[i + 4..i + 8].try_into().unwrap();
            let ctype = ctype.map(|f| f as char);

            let cdata = &bytes[i + 8..i + 8 + clength as usize];
            let _crc = &bytes[i + 8 + clength as usize..i + 8 + clength as usize + 4];

            if parse::parse_chunk(ctype, cdata, &mut construct, &mut pixel_data) {
                break;
            }

            i += 8 + clength as usize + 4;
        }

        parse::parse_idat_all(&pixel_data, &mut construct);

        construct
    }
}
