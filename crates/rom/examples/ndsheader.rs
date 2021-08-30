use std::env;
use std::process;

fn main() {
    let file = match env::args_os().nth(1) {
        Some(file) => file,
        None => {
            println!("usage: ndsheader <rom>");
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

    println!("{}", &rom.header);
}
