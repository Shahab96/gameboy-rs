use std::array::TryFromSliceError;
use std::error::Error;
use std::fmt::Display;
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Debug)]
pub enum ChecksumType {
    Global,
    Header,
}

#[derive(Debug)]
pub enum CartridgeError {
    InvalidLength,
    InvalidFile,
    InvalidNintendoLogo,
    BadChecksum(ChecksumType),
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
            CartridgeError::InvalidNintendoLogo => write!(f, "Invalid Nintendo logo"),
            CartridgeError::BadChecksum(ChecksumType::Header) => write!(f, "Bad header checksum"),
            CartridgeError::BadChecksum(ChecksumType::Global) => write!(f, "Bad global checksum"),
        }
    }
}

impl Error for CartridgeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self)
    }
}

#[derive(Debug)]
pub struct CartridgeHeader {
    pub entry: [u8; 4],
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

    pub data: Vec<u8>,
}

impl Display for CartridgeHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CartridgeHeader{{\n\tEntry: {:02X?}\n\tTitle: {:?}\n\tManufacturer Code: {:?}\n\tCGB Flag: {:?}\n\tNew Licensee Code: {:?}\n\tSGB Flag: {:?}\n\tCartridge Type: {:?}\n\tROM Size: {:?}\n\tRAM Size: {:?}\n\tDestination Code: {:?}\n\tOld Licensee Code: {:?}\n\tMask ROM Version Number: {:?}\n\tHeader Checksum: {:?}\n\tGlobal Checksum: {:?}\n}}",
            self.entry,
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

const NINTENDO_LOGO: [u8; 48] = [
    0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D,
    0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99,
    0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
];

impl Into<Vec<u8>> for CartridgeHeader {
    fn into(self) -> Vec<u8> {
        self.data
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
            Ok(_) => {
                if data[0x104..0x134] != NINTENDO_LOGO {
                    return Err(CartridgeError::InvalidNintendoLogo);
                }

                Ok(data)
            }
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

        dbg!("Calculated checksum: {:04X}", sum);
        dbg!("Provided checksum: {:04X}", checksum);

        sum == checksum
    }

    pub fn load(path: &Path) -> Result<Self, CartridgeError> {
        let data = CartridgeHeader::read_file(path)?;

        let mut entry = [0u8; 4];
        let mut title = [0u8; 16];
        let mut manufacturer_code = [0u8; 4];
        let mut new_licensee_code = [0u8; 2];
        let mut global_checksum = [0u8; 2];

        entry.copy_from_slice(&data[0x100..0x104]);
        title.copy_from_slice(&data[0x134..0x144]);
        manufacturer_code.copy_from_slice(&data[0x13F..0x143]);
        new_licensee_code.copy_from_slice(&data[0x144..0x146]);
        global_checksum.copy_from_slice(&data[0x14E..0x150]);

        if !CartridgeHeader::header_checksum(data.as_slice(), &data[0x14D]) {
            return Err(CartridgeError::BadChecksum(ChecksumType::Header));
        }

        if !CartridgeHeader::global_checksum(data.as_slice(), &global_checksum) {
            return Err(CartridgeError::BadChecksum(ChecksumType::Global));
        }

        dbg!("Cartridge size: {} bytes", data.len());

        let cartridge_header = CartridgeHeader {
            entry,
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
            data,
        };

        Ok(cartridge_header)
    }
}
