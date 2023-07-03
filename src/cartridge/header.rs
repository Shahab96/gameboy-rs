use std::array::TryFromSliceError;
use std::error::Error;
use std::fmt::Display;
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Debug)]
pub enum CartridgeError {
    InvalidLength,
    InvalidFile,
    BadChecksum,
}

impl From<TryFromSliceError> for CartridgeError {
    fn from(e: TryFromSliceError) -> Self {
        eprintln!("Error: {}", e);
        CartridgeError::InvalidFile
    }
}

impl Display for CartridgeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CartridgeError::InvalidLength => write!(f, "Invalid length"),
            CartridgeError::InvalidFile => write!(f, "Invalid file"),
            CartridgeError::BadChecksum => write!(f, "Bad checksum"),
        }
    }
}

impl Error for CartridgeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            CartridgeError::InvalidLength => Some(self),
            CartridgeError::InvalidFile => Some(self),
            CartridgeError::BadChecksum => Some(self),
        }
    }
}

#[derive(Debug)]
pub struct CartridgeHeader {
    pub entry: [u8; 4],
    pub nintendo_logo: [u8; 48],
    pub title: [u8; 16],
    pub manufacturer_code: [u8; 4],
    pub cgb_flag: Option<u8>,
    pub new_licensee_code: [u8; 2],
    pub sgb_flag: u8,
    pub cartridge_type: u8,
    pub rom_size: u8,
    pub ram_size: u8,
    pub destination_code: u8,
    pub old_licensee_code: u8,
    pub mask_rom_version_number: u8,
    pub header_checksum: u8,
    pub global_checksum: [u8; 2],
}

impl Display for CartridgeHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Entry: {:?}\nNintendo Logo: {:?}\nTitle: {:?}\nManufacturer Code: {:?}\nCGB Flag: {:?}\nNew Licensee Code: {:?}\nSGB Flag: {:?}\nCartridge Type: {:?}\nROM Size: {:?}\nRAM Size: {:?}\nDestination Code: {:?}\nOld Licensee Code: {:?}\nMask ROM Version Number: {:?}\nHeader Checksum: {:?}\nGlobal Checksum: {:?}",
            self.entry,
            self.nintendo_logo,
            self.title,
            self.manufacturer_code,
            self.cgb_flag,
            self.new_licensee_code,
            self.sgb_flag,
            self.cartridge_type,
            self.rom_size,
            self.ram_size,
            self.destination_code,
            self.old_licensee_code,
            self.mask_rom_version_number,
            self.header_checksum,
            self.global_checksum,
        )
    }
}

impl CartridgeHeader {
    fn read_file(path: &Path) -> Result<Vec<u8>, CartridgeError> {
        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(_) => {
                return Err(CartridgeError::InvalidFile);
            }
        };

        let file_length = match file.metadata() {
            Ok(metadata) => metadata.len(),
            Err(_) => {
                return Err(CartridgeError::InvalidFile);
            }
        };

        if file_length < 0x8000 || file_length % 0x4000 != 0 {
            return Err(CartridgeError::InvalidLength);
        }

        let mut data = vec![];

        match file.read_to_end(&mut data) {
            Ok(_) => Ok(data),
            Err(_) => Err(CartridgeError::InvalidFile),
        }
    }

    fn header_checksum(data: &[u8], checksum: &u8) -> bool {
        let mut sum = 0u8;

        for i in 0x134..0x14D {
            sum = sum.wrapping_sub(data[i]).wrapping_sub(1);
        }

        &sum == checksum
    }

    fn global_checksum(data: &[u8], checksum: &[u8; 2]) -> bool {
        let mut sum = 0u16;
        let checksum = (checksum[0] as u16) << 8 | checksum[1] as u16;

        for i in 0..data.len() {
            if i == 0x14E || i == 0x14F {
                continue;
            }

            sum = sum.wrapping_add(data[i] as u16);
        }

        sum == checksum
    }

    pub fn load(path: &Path) -> Result<Self, CartridgeError> {
        let data = CartridgeHeader::read_file(path)?;

        let mut entry = [0u8; 4];
        let mut nintendo_logo = [0u8; 48];
        let mut title = [0u8; 16];
        let mut manufacturer_code = [0u8; 4];
        let mut new_licensee_code = [0u8; 2];
        let mut global_checksum = [0u8; 2];

        entry.copy_from_slice(&data[0x100..0x104]);
        nintendo_logo.copy_from_slice(&data[0x104..0x134]);
        title.copy_from_slice(&data[0x134..0x144]);
        manufacturer_code.copy_from_slice(&data[0x13F..0x143]);
        new_licensee_code.copy_from_slice(&data[0x144..0x146]);
        global_checksum.copy_from_slice(&data[0x14E..0x150]);

        let cartridge_header = CartridgeHeader {
            entry,
            nintendo_logo,
            title,
            manufacturer_code,
            cgb_flag: match data[0x143] {
                0x80 | 0xC0 => Some(data[0x143]),
                _ => None,
            },
            new_licensee_code,
            sgb_flag: data[0x146],
            cartridge_type: data[0x147],
            rom_size: data[0x148],
            ram_size: data[0x149],
            destination_code: data[0x14A],
            old_licensee_code: data[0x14B],
            mask_rom_version_number: data[0x14C],
            header_checksum: data[0x14D],
            global_checksum,
        };

        if !CartridgeHeader::header_checksum(data.as_slice(), &cartridge_header.header_checksum) {
            return Err(CartridgeError::BadChecksum);
        }

        if !CartridgeHeader::global_checksum(data.as_slice(), &global_checksum) {
            return Err(CartridgeError::BadChecksum);
        }

        Ok(cartridge_header)
    }
}
