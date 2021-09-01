use std::env;
use std::process;

fn main() {
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

    // TODO: Better approach to dumping ROM information.

    println!("Header information:\n{}", &rom.header);

    if let Some(banner) = rom.banner {
        println!();

        // TODO: Add support for other header versions.
        println!("Banner CRC:            {:#06X}", banner.crc16[0]);

        let title = banner.title_english.to_string_lossy();
        for (i, line) in title.split('\n').enumerate() {
            println!("Banner text, line {}:   {}", i + 1, line,);
        }
    }
}
