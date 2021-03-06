use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

use common::util::{crc, FileSize};

mod banner;
mod header;
mod info;

pub mod encrypt;

use self::encrypt::Key1;
use self::info::{MemoryKind, RomParams, SramKind};

pub use self::banner::NdsBanner;
pub use self::header::NdsHeader;

/// NDS ROM.
#[derive(Debug)]
pub struct NdsRom {
    /// The ROM data.
    pub rom: Box<[u8]>,
    /// A copy of the ROM header.
    pub header: NdsHeader,
    /// A copy of the ROM banner, if it exists.
    pub banner: Option<NdsBanner>,
    /// Extra information about the ROM.
    pub params: RomParams,
    /// A generated chip ID for the ROM.
    pub chip_id: u32,
}

impl NdsRom {
    // TODO: Split up this function into smaller functions.
    fn load_data(rom: Vec<u8>, rom_data_size: usize) -> NdsRom {
        let rom = rom.into_boxed_slice();
        let rom_size = rom.len();

        let header = NdsHeader::read(&rom);
        let banner = match header.banner_offset {
            0 => None,
            offset => Some(NdsBanner::read(&rom, offset as usize)),
        };

        let game_code = header.game_code();

        let params = match RomParams::get(game_code) {
            Some(&params) => {
                log::info!(
                    "ROM entry: {} (SRAM {})",
                    FileSize(params.rom_size as usize),
                    params.sram_kind,
                );

                params
            }
            None => {
                let sram_kind = if header.is_homebrew() {
                    // No SRAM for homebrew.
                    SramKind::None
                } else {
                    // FIXME: We assume EEPROM with 64KB (same behaviour as melonDS).
                    SramKind::Eeprom64KB
                };

                RomParams {
                    rom_size: rom_size as u32,
                    sram_kind,
                }
            }
        };

        if params.rom_size as usize != rom_data_size {
            log::warn!(
                "bad ROM size {} (expected {}), rounded to {}",
                rom_data_size,
                header.rom_size,
                rom_size
            );
        }

        // Generate ROM chip ID.
        //
        // Note: Most games wont check the value, it just needs to be consistent.
        //
        //   1st byte - Manufacturer (eg. C2h=Macronix) (roughly based on JEDEC IDs)
        //   2nd byte - Chip size (00h..7Fh: (N+1)MB, F0h..FFh: (100h-N)*256MB?)
        //   3rd byte - Flags (see below)
        //   4th byte - Flags (see below)
        //
        // The Flag Bits in 3th byte can be
        //
        //   0   Uses Infrared (but via SPI, unrelated to ROM) (also Jam with the Band)
        //   1   Unknown (set in some 3DS carts)
        //   2-6 Zero
        //   7   Unknown (set in Kingdom Hearts - Re-Coded)
        //
        // The Flag Bits in 4th byte can be
        //
        //   0-2 Zero
        //   3   NAND flag (0=ROM, 1=NAND)
        //   4   3DS Flag  (0=NDS/DSi, 1=3DS)
        //   5   Unknown   (0=Normal, 1=Support cmd B5h/D6h)
        //   6   DSi flag  (0=NDS/3DS, 1=DSi) (but also set in NDS Walk with Me)
        //   7   Cart Protocol Variant (0=old/smaller MROM, 1=new/bigger 1T-ROM or NAND)
        let mut chip_id = 0x000000C2;

        if rom_size >= 256 * 1024 * 1024 {
            chip_id |= (0x100 - (rom_size as u32 >> 28)) << 8;
        } else if 1024 * 1024 <= rom_size && rom_size <= 128 * 1024 * 1024 {
            chip_id |= ((rom_size as u32 >> 20) - 1) << 8;
        } else {
            log::warn!("unexpected ROM size: {:#X}", rom_size);
        }

        if header.is_dsi() {
            chip_id |= 0x08000000;
        }
        if params.sram_kind.memory_kind() == MemoryKind::Nand {
            chip_id |= 0x48000000;
        } else if params.rom_size >= 128 * 1024 * 1024 {
            chip_id |= 0x80000000;
        }

        log::info!("ROM chip ID: {:#010X}", chip_id);

        // log::info!(
        //     "Action Replay game ID: {}-{:08X}",
        //     header.game_code,
        //     crc::crc32(&rom[..0x200])
        // );

        let mut rom = NdsRom {
            rom,
            header,
            banner,
            params,
            chip_id,
        };

        rom.init_secure_area(game_code);

        rom
    }

    fn init_secure_area(&mut self, game_code: u32) {
        // The secure area exists if the ARM9 boot code ROM `offset` is located
        // within `0x4000..0x8000`. If so, it will be loaded (by BIOS via KEY1
        // encrypted commands) in 4KB portions, starting at `offset`, aligned to
        // `0x1000`, up to address `0x7FFF`.
        //
        // The secure area size is thus `0x8000 - offset`, regardless of the
        // ARM9 boot code size entry in the header.
        //
        // Note: The BIOS silently skips and ARM9 boot code when `offset < 0x4000`.
        //
        // <https://problemkaputt.de/gbatek.htm#dscartridgesecurearea>
        // if header.has_secure_area() {
        if let Some(secure_area) = self.secure_area_mut() {
            // The first 8 bytes of the secure area contain the secure area ID,
            // this ID is verified by the BIOS boot code, the ID value changes
            // during the boot process:
            //
            //   "encryObj"                raw ID before encryption (raw ROM image)
            //   (encrypted)               encrypted ID after encryption (encrypted ROM image)
            //   "encryObj"                raw ID before encryption (verified by BIOS boot code)
            //   0xE7FFDEFF, 0xE7FFDEFF    destroyed ID (overwritten by BIOS after verifying)
            //
            // If the decrypted ID does match, then the BIOS overwrites the first
            // 8 bytes with 0xE7FFDEFF values (ie. only the ID is destroyed). If
            // the ID doesn't match, then the first 0x800 bytes (2KB) are overwritten
            // by 0xE7FFDEFF values.

            // Magic value for destroyed secure area ID.
            // This is a little endian u32 value.
            const E7FFDEFF: [u8; 4] = [0xFF, 0xDE, 0xFF, 0xE7];

            // Re-encrypt secure area if needed.
            if secure_area[0..4] == E7FFDEFF && secure_area[0x10..0x14] != E7FFDEFF {
                log::debug!("re-encrypting ROM secure area");

                Key1::encrypt_secure_area(secure_area, game_code);
            }
        }
    }

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
        // Read the ROM file into the buffer.
        {
            let mut buf = &mut rom[..];
            loop {
                match file.read(&mut buf) {
                    Ok(0) => break,
                    Ok(n) => {
                        assert!(n <= buf.len());
                        buf = &mut buf[n..];
                    }
                    Err(ref err) if err.kind() == io::ErrorKind::Interrupted => {}
                    Err(err) => return Err(err),
                }
            }
        }

        Ok(Self::load_data(rom, len))
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

        Ok(Self::load_data(rom, len))
    }

    /// Returns `true` if the ROM a homebrew.
    #[inline]
    pub fn is_homebrew(&self) -> bool {
        self.header.is_homebrew()
    }

    /// Returns `true` if the ROM is a DSi ROM.
    #[inline]
    pub fn is_dsi(&self) -> bool {
        self.header.is_dsi()
    }

    /// Returns `true` if the ROM has Infrared (IR).
    #[inline]
    pub fn has_ir(&self) -> bool {
        self.header.has_ir()
    }

    /// Returns `true` if the ROM has a secure area.
    #[inline]
    pub fn has_secure_area(&self) -> bool {
        self.header.has_secure_area()
    }

    /// Returns the game code as a `u32`.
    #[inline]
    pub fn game_code(&self) -> u32 {
        self.header.game_code()
    }

    /// Returns a reference the secure area, if it exists.
    pub fn secure_area(&self) -> Option<&[u8]> {
        if self.header.has_secure_area() {
            Some(&self.rom[(self.header.arm9_rom_offset as usize)..0x8000])
        } else {
            None
        }
    }

    /// Returns a mutable reference the secure area, if it exists.
    pub fn secure_area_mut(&mut self) -> Option<&mut [u8]> {
        if self.header.has_secure_area() {
            Some(&mut self.rom[(self.header.arm9_rom_offset as usize)..0x8000])
        } else {
            None
        }
    }

    /// Computes the secure area checksum, if it exists.
    pub fn compute_secure_area_crc16(&self) -> Option<u16> {
        self.secure_area().map(crc::crc16)
    }
}
