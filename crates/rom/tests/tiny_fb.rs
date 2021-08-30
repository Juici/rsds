use rom::nds::NdsRom;

const TINY_FB: &[u8] = include_bytes!("../../../demo/TinyFB.nds");

#[test]
fn read_rom() {
    let rom = NdsRom::load(TINY_FB).unwrap();

    let header = &rom.header;
    assert_eq!(header.game_title, "NDS.TinyFB");
    assert_eq!(header.game_code, "####");
    assert_eq!(header.compute_logo_crc16(), 0x9E1A);
    assert_eq!(header.compute_header_crc16(), 0x908E);

    assert!(rom.banner.is_none());
}
