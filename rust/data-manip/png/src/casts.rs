pub fn cast_2u8_u16(x: &[u8]) -> u16 {
    unsafe { std::ptr::read_unaligned(x as *const [u8] as *const u16) }.to_be()
}

pub fn cast_4u8_u32(x: &[u8]) -> u32 {
    unsafe { std::ptr::read_unaligned(x as *const [u8] as *const u32) }.to_be()
}

#[macro_export]
macro_rules! cast_tuple_4u8_u32 {
    ($($x:expr $(,)?)+) => {
        ($(
        cast_4u8_u32($x)
        ,)+)
    };
}
