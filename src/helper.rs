use super::mnist::Dataset;

pub fn slice_to_u32(slice: &[u8]) -> u32 {
    u32::from_be_bytes([slice[0], slice[1], slice[2], slice[3]])
}
