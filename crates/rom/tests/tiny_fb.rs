use rom::nds::Header;

const TINY_FB: &[u8] = include_bytes!("../../../demo/TinyFB.nds");

#[test]
fn header() {
    let rom = TINY_FB;
    let header = Header::from_reader(rom).unwrap();

    assert_eq!(header.game_title, "NDS.TinyFB");
    assert_eq!(header.game_code, "####");
    assert_eq!(header.compute_logo_crc16(), 0x9E1A);
    assert_eq!(header.compute_header_crc16(), 0x908E);
}
