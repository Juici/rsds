use rom::nds::Header;

const TINY_FB: &[u8] = include_bytes!("../../../demo/TinyFB.nds");

#[test]
fn header() {
    let header = unsafe { *(TINY_FB as *const [u8] as *const Header) };

    let mut dump = String::new();
    header.dump(&mut dump).unwrap();

    eprintln!("{}", dump);
    panic!();
}
