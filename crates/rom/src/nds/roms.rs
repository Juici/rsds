//! A list of ROMs along with their size and SRAM kind.

use std::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SramKind {
    /// No SRAM.
    None = 0,
    /// 512B EEPROM.
    Eeprom512B = 1,
    /// 8KB EEPROM.
    Eeprom8KB = 2,
    /// 64KB EEPROM.
    Eeprom64KB = 3,
    /// 128KB EEPROM.
    Eeprom128KB = 4,
    /// 256KB Flash.
    Flash256KB = 5,
    /// 512KB Flash.
    Flash512KB = 6,
    /// 1MB Flash.
    Flash1MB = 7,
    /// 8MB NAND.
    Nand8MB = 8,
    /// 16MB NAND.
    Nand16MB = 9,
    /// 64MB NAND.
    Nand64MB = 10,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MemoryKind {
    /// No SRAM.
    None,
    /// Small EEPROM.
    EepromSmall,
    /// Regular EEPROM.
    EepromRegular,
    /// Flash.
    Flash,
    /// NAND.
    Nand,
}

/// A ROM with known size and SRAM.
#[derive(Clone, Copy, Debug)]
pub struct Entry {
    /// The size of the ROM in bytes.
    pub rom_size: u32,
    /// The kind and size of SRAM.
    pub sram_kind: SramKind,
}

impl SramKind {
    /// Returns the size of the SRAM in bytes.
    pub fn size(self) -> usize {
        match self {
            SramKind::None => 0,
            SramKind::Eeprom512B => 512,
            SramKind::Eeprom8KB => 8 * 1024,
            SramKind::Eeprom64KB => 64 * 1024,
            SramKind::Eeprom128KB => 128 * 1024,
            SramKind::Flash256KB => 256 * 1024,
            SramKind::Flash512KB => 512 * 1024,
            SramKind::Flash1MB => 1024 * 1024,
            SramKind::Nand8MB => 8 * 1024 * 1024,
            SramKind::Nand16MB => 16 * 1024 * 1024,
            SramKind::Nand64MB => 64 * 1024 * 1024,
        }
    }

    /// Returns the kind of memory.
    pub fn memory_kind(self) -> MemoryKind {
        match self {
            SramKind::None => MemoryKind::None,
            SramKind::Eeprom512B => MemoryKind::EepromSmall,
            SramKind::Eeprom8KB | SramKind::Eeprom64KB | SramKind::Eeprom128KB => {
                MemoryKind::EepromRegular
            }
            SramKind::Flash256KB | SramKind::Flash512KB | SramKind::Flash1MB => MemoryKind::Flash,
            SramKind::Nand8MB | SramKind::Nand16MB | SramKind::Nand64MB => MemoryKind::Nand,
        }
    }
}

impl fmt::Display for SramKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            SramKind::None => "None",
            SramKind::Eeprom512B => "EEPROM 512B",
            SramKind::Eeprom8KB => "EEPROM 8KB",
            SramKind::Eeprom64KB => "EEPROM 64KB",
            SramKind::Eeprom128KB => "EEPROM 128KB",
            SramKind::Flash256KB => "Flash 256KB",
            SramKind::Flash512KB => "Flash 512KB",
            SramKind::Flash1MB => "Flash 1MB",
            SramKind::Nand8MB => "NAND 8MB",
            SramKind::Nand16MB => "NAND 16MB",
            SramKind::Nand64MB => "NAND 64MB",
        })
    }
}

pub static ROMS: phf::Map<u32, Entry> = include!(concat!(env!("OUT_DIR"), "/roms_map"));
