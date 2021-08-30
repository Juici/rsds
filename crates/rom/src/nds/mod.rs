use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

mod banner;
mod header;

pub use self::banner::NdsBanner;
pub use self::header::NdsHeader;

/// NDS ROM.
pub struct NdsRom {
    pub rom: Box<[u8]>,
    pub header: NdsHeader,
    pub banner: Option<NdsBanner>,
}

impl NdsRom {
    /// Loads a ROM from a file.
    pub fn open<P: AsRef<Path>>(path: P) -> io::Result<NdsRom> {
        let mut file = File::open(path)?;

        let meta = file.metadata()?;
        let len = meta.len() as usize;

        // ROM should be at least as large as the header.
        let mut rom_size = NdsHeader::SIZE;
        while rom_size < len {
            rom_size <<= 1;
        }

        let mut rom = vec![0u8; rom_size];
        file.read_to_end(&mut rom)?;

        // Close the file.
        drop(file);

        Ok(Self::load_data(rom))
    }

    /// Loads a ROM from a byte array.
    pub fn load(bytes: &[u8]) -> io::Result<NdsRom> {
        let len = bytes.len();

        // ROM should be at least as large as the header.
        let mut rom_size = NdsHeader::SIZE;
        while rom_size < len {
            rom_size <<= 1;
        }

        let mut rom = vec![0u8; rom_size];
        rom[..len].copy_from_slice(bytes);

        Ok(Self::load_data(rom))
    }

    fn load_data(rom: Vec<u8>) -> NdsRom {
        let header = NdsHeader::read(&rom);
        let banner = match header.banner_offset {
            0 => None,
            offset => Some(NdsBanner::read(&rom, offset as usize)),
        };

        NdsRom {
            rom: rom.into_boxed_slice(),
            header,
            banner,
        }
    }
}
