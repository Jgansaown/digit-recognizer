pub fn slice_to_u32(slice: &[u8]) -> u32 {
    return u32::from_be_bytes([slice[0], slice[1], slice[2], slice[3]]);
    // let mut ans = 0;
    // for byte in slice {
    //     ans = (ans << 8) + (*byte as u32);
    // }
    // ans
}
