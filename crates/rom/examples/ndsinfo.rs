use std::env;
use std::error::Error;
use std::io::{self, Write};
use std::process;

use common::util::FileSize;
use rom::nds::{NdsBanner, NdsRom};

fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::try_init_custom_env("RSDS_LOG").unwrap();

    let file = match env::args_os().nth(1) {
        Some(file) => file,
        None => {
            println!("usage: ndsinfo <rom>");
            process::exit(0);
        }
    };

    let rom = match rom::nds::NdsRom::open(file) {
        Ok(rom) => rom,
        Err(err) => {
            eprintln!("error: {}", err);
            process::exit(1);
        }
    };

    let stdout = io::stdout();
    let mut f = stdout.lock();

    header_info(&mut f, &rom)?;

    if let Some(banner) = &rom.banner {
        writeln!(f)?;
        banner_info(&mut f, banner)?;
    }

    f.flush()?;

    Ok(())
}

#[rustfmt::skip]
fn header_info<W: io::Write>(mut f: W, rom: &NdsRom) -> io::Result<()> {
    let header = &rom.header;

    macro_rules! w {
        ($addr:literal, $name:literal) => {
            write!(f, concat!($addr, "  ", $name))
        };
        ($addr:literal, $name:expr, $fmt:literal, $($value:expr),*) => {
            write!(f, concat!($addr, "  {:40}  ", $fmt), $name, $($value),*)
        };
    }

    writeln!(f, "Header information:")?;

    w!("0x000", "Game title", "{}\n", header.game_title)?;

    w!("0x00C", "Game code", "{}", header.game_code)?;
    if let Some(region) = header.region() {
        write!(f, " (NTR-{}-{})", header.game_code, region)?;
    }
    writeln!(f)?;

    w!("0x010", "Maker code", "{}", header.maker_code)?;
    if let Some(maker) = header.maker() {
        write!(f, " ({})", maker)?;
    }
    writeln!(f)?;

    w!("0x012", "Unit code", "{:#04X}\n", header.unit_code)?;
    w!("0x013", "Device type", "{:#04X}\n", header.device_type)?;
    w!("0x014", "Device capacity", "{:#04X} ({})\n", header.device_capacity, FileSize(header.device_capacity_bytes()))?;
    w!("0x015", "(8 bytes reserved)\n")?;
    w!("0x01D", "NDS region", "{:#04X}\n", header.nds_region)?;
    w!("0x01E", "ROM version", "{:#04X}\n", header.rom_version)?;
    w!("0x01F", "Autostart", "{:#04X}\n", header.autostart)?;

    w!("0x020", "ARM9 ROM offset", "{:#X}\n", header.arm9_rom_offset)?;
    w!("0x024", "ARM9 entry address", "{:#X}\n", header.arm9_entry_address)?;
    w!("0x028", "ARM9 RAM address", "{:#X}\n", header.arm9_ram_address)?;
    w!("0x02C", "ARM9 code size", "{:#X}\n", header.arm9_size)?;

    w!("0x030", "ARM7 ROM offset", "{:#X}\n", header.arm7_rom_offset)?;
    w!("0x034", "ARM7 entry address", "{:#X}\n", header.arm7_entry_address)?;
    w!("0x038", "ARM7 RAM address", "{:#X}\n", header.arm7_ram_address)?;
    w!("0x03C", "ARM7 code size", "{:#X}\n", header.arm7_size)?;

    w!("0x040", "File name table (FNT) offset", "{:#X}\n", header.fnt_offset)?;
    w!("0x044", "File name table (FNT) size", "{:#X}\n", header.fnt_size)?;
    w!("0x048", "File allocation table (FAT) offset", "{:#X}\n", header.fat_offset)?;
    w!("0x04C", "File allocation table (FAT) size", "{:#X}\n", header.fat_size)?;

    w!("0x050", "ARM9 overlay offset", "{:#X}\n", header.arm9_overlay_offset)?;
    w!("0x054", "ARM9 overlay size", "{:#X}\n", header.arm9_overlay_size)?;
    w!("0x058", "ARM7 overlay offset", "{:#X}\n", header.arm7_overlay_offset)?;
    w!("0x05C", "ARM7 overlay size", "{:#X}\n", header.arm7_overlay_size)?;

    w!("0x060", "Normal commands settings", "{:#010X}\n", header.normal_command_settings)?;
    w!("0x064", "KEY1 commands settings", "{:#010X}\n", header.key1_command_settings)?;

    w!("0x068", "Banner offset", "{:#X}\n", header.banner_offset)?;

    let secure_area_crc = match rom.compute_secure_area_crc16() {
        Some(crc) if crc == header.secure_area_crc16 => "OK",
        Some(_) => "INVALID",
        None => "-",
    };
    w!("0x06C", "Secure area CRC", "{:#06X} ({})\n", header.secure_area_crc16, secure_area_crc)?;

    let delay_ms = header.secure_area_delay as f64 / 131.0;
    w!("0x06E", "Secure area delay", "{:#06X} ({:.0} ms)\n", header.secure_area_delay, delay_ms)?;

    w!("0x070", "ARM9 autoload hook RAM address?", "{:#X}\n", header.arm9_autoload)?;
    w!("0x074", "ARM7 autoload hook RAM address?", "{:#X}\n", header.arm7_autoload)?;

    w!("0x078", "Secure area disable", "{:#018X}\n", header.secure_area_disable)?;

    w!("0x080", "ROM size", "{:#X}\n", header.rom_size)?;
    w!("0x084", "ROM header size", "{:#X}\n", header.header_size)?;

    w!("0x088", "(4 bytes unknown)\n")?;
    w!("0x08C", "(8 bytes reserved)\n")?;

    w!("0x094", "NAND end of ROM area", "{:#06X}\n", header.nand_rom_end)?;
    w!("0x096", "NAND start of RW area", "{:#06X}\n", header.nand_rw_start)?;

    w!("0x098", "(40 bytes reserved)\n")?;

    let logo_crc = if header.compute_logo_crc16() == header.nintendo_logo_crc16 { "OK" } else { "INVALID" };
    let header_crc = if header.compute_header_crc16() == header.header_crc16 { "OK" } else { "INVALID" };

    w!("0x0C0", "Nintendo logo (156 bytes)\n")?;
    w!("0x15C", "Nintendo logo CRC", "{:#06X} ({})\n", header.nintendo_logo_crc16, logo_crc)?;
    w!("0x15E", "Header CRC", "{:#06X} ({})\n", header.header_crc16, header_crc)?;

    w!("0x160", "Debug ROM offset", "{:#X}\n", header.debug_rom_offset)?;
    w!("0x164", "Debug code size", "{:#X}\n", header.debug_size)?;
    w!("0x168", "Debug RAM address", "{:#X}\n", header.debug_ram_address)?;

    w!("0x16C", "(4 bytes reserved)\n")?;
    w!("0x170", "(144 bytes reserved)\n")?;

    Ok(())
}

#[rustfmt::skip]
fn banner_info<W: io::Write>(mut f: W, banner: &NdsBanner) -> io::Result<()> {
    macro_rules! w {
        ($name:expr, $fmt:literal, $($value:expr),*) => {
            write!(f, concat!("{:47}  ", $fmt), $name, $($value),*)
        };
    }

    w!("Banner CRC:", "{:#06X}\n", banner.crc16[0])?;

    for (i, line) in banner.title_english.to_string_lossy().split('\n').enumerate() {
        let desc = format!("English banner text, line {}:", i + 1);
        w!(desc, "{}\n", line)?;
    }

    Ok(())
}
