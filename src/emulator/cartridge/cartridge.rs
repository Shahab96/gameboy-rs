pub struct Cartridge {
    pub data: [u8; 0x8000],
}

impl From<Vec<u8>> for Cartridge {
    fn from(data: Vec<u8>) -> Self {
        let mut cartridge = Cartridge { data: [0; 0x8000] };
        cartridge.data[..data.len()].copy_from_slice(&data);
        cartridge
    }
}
