use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

use serde::Deserialize;

const ROMS: &str = include_str!("data/rom_list.toml");

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    generate_roms_list().unwrap();
}

#[derive(Deserialize)]
struct RomsList {
    roms: Vec<Rom>,
}

#[derive(Deserialize)]
struct Rom {
    game_code: u32,
    rom_size: u32,
    sram_kind: u32,
}

fn generate_roms_list() -> Result<(), Box<dyn Error>> {
    let roms: RomsList = toml::from_str(ROMS)?;

    let mut map = phf_codegen::Map::new();
    for rom in roms.roms {
        let sram = match rom.sram_kind {
            0 => "None",
            1 => "Eeprom512B",
            2 => "Eeprom8KB",
            3 => "Eeprom64KB",
            4 => "Eeprom128KB",
            5 => "Flash256KB",
            6 => "Flash512KB",
            7 => "Flash1MB",
            8 => "Nand8MB",
            9 => "Nand16MB",
            10 => "Nand64MB",
            n => {
                println!(
                    "cargo:warning=unknown SRAM type {:#010X} for game code {:#010X}",
                    n, rom.game_code,
                );
                "None"
            }
        };

        let entry = format!(
            "RomParams {{ rom_size: {:#010X}, sram_kind: SramKind::{} }}",
            rom.rom_size, sram
        );

        map.entry(rom.game_code, &entry);
    }
    let map = map.build();

    let mut path = PathBuf::from(env::var("OUT_DIR")?);
    path.push("roms_map");

    let mut file = BufWriter::new(File::create(&path)?);
    write!(file, "{}", map)?;

    Ok(())
}
