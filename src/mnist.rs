#[derive(Debug)]
struct Header {
    magic: u32,
    nums: u32,
    rows: u32,
    cols: u32,
}
impl Header {
    fn new(bytes: &[u8]) -> Self {
        Self {
            magic: u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]),
            nums: u32::from_be_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]),
            rows: u32::from_be_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]),
            cols: u32::from_be_bytes([bytes[12], bytes[13], bytes[14], bytes[15]]),
        }
    }
}

#[derive(Debug)]
pub struct Dataset {
    pub nums: u32,
    pub rows: u32,
    pub cols: u32,
    data: Vec<u8>,
    labels: Vec<u8>,
}
impl Dataset {
    pub fn new(data: Vec<u8>, labels: Vec<u8>) -> Self {
        let h1 = Header::new(&data);
        let h2 = Header::new(&labels);

        assert!(h1.nums == h2.nums);

        println!("H1: {:?}, H2: {:?}", h1, h2);

        Self {
            nums: h1.nums,
            rows: h1.rows,
            cols: h1.cols,
            data,
            labels,
        }
    }
}
