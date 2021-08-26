/// NDS cartridge header.
///
/// Loaded from `0x00` in ROM to `0x27FFE00` on power-up.
///
/// <https://problemkaputt.de/gbatek.htm#dscartridgesencryptionfirmware>
#[repr(C)]
pub struct Header {
    /// Game title.
    ///
    /// Uppercase ASCII, padded with `0x00`.
    pub game_title: [u8; 12], // 0x000
    /// Game code.
    ///
    /// Uppercase ASCII, `NTR-{code}`.
    pub game_code: [u8; 4], // 0x00C
    /// Maker code.
    ///
    /// Uppercase ASCII, eg. `01` is Nintendo.
    pub maker_code: [u8; 2], // 0x010
    /// Unit code.
    ///
    /// - `0x00` = NDS
    /// - `0x02` = NDS + DSi
    /// - `0x03` = DSi
    pub unit_code: u8, // 0x012
    /// Encryption seed select.
    ///
    /// `0x00..=0x07`, usually `0x00`.
    pub encryption_seed_select: u8, // 0x013
    /// Device capacity.
    ///
    /// `chip size = 128KB << capacity`.
    pub device_capacity: u8, // 0x014
    /// Reserved, zero filled.
    reserved1: [u8; 8], // 0x015
    /// NDS region.
    ///
    /// - `0x00` = Normal
    /// - `0x40` = Korea
    /// - `0x80` = China
    pub nds_region: u8, // 0x01D
    /// ROM version.
    ///
    /// Usually `0x00`.
    pub rom_version: u8, // 0x01E
    /// Autostart.
    ///
    /// Bit2: Skip "Press Button" after Health and Safety.
    ///
    /// Also skips bootmenu, even in Manual mode and even with Start pressed.
    pub autostart: u8, // 0x01F

    /// ARM9 ROM offset.
    pub arm9_rom_offset: u32, // 0x020
    /// ARM9 entry address.
    ///
    /// `0x2000000..=0x23BFE00`.
    pub arm9_entry_address: u32, // 0x024
    /// ARM9 RAM address.
    ///
    /// `0x2000000..=0x23BFE00`.
    pub arm9_ram_address: u32, // 0x028
    /// ARM9 code size.
    ///
    /// Max `0x3BFE00` (3839.5KB).
    pub arm9_size: u32, // 0x02C

    /// ARM7 ROM offset.
    pub arm7_rom_offset: u32, // 0x030
    /// ARM7 entry address.
    ///
    /// `0x2000000..=0x23BFE00`, or `0x37F8000..=0x3807E00`.
    /// ARM7 RAM address.
    pub arm7_entry_address: u32, // 0x034
    ///
    /// `0x2000000..=0x23BFE00`, or `0x37F8000..=0x3807E00`.
    pub arm7_ram_address: u32, // 0x038
    /// ARM7 code size.
    ///
    /// Max `0x3BFE00` (3839.5KB) or `0xFE00` (63.5KB).
    pub arm7_size: u32, // 0x03C

    /// File name table (FNT) offset.
    pub fnt_offset: u32, // 0x040
    /// File name table (FNT) size.
    pub fnt_size: u32, // 0x044
    /// File allocation table (FAT) offset.
    pub fat_offset: u32, // 0x048
    /// File allocation table (FAT) size.
    pub fat_size: u32, // 0x04C

    /// ARM9 overlay offset.
    pub arm9_overlay_offset: u32, // 0x050
    /// ARM9 overlay size.
    pub arm9_overlay_size: u32, // 0x054
    /// ARM7 overlay offset.
    pub arm7_overlay_offset: u32, // 0x058
    /// ARM7 overlay size.
    pub arm7_overlay_size: u32, // 0x05C

    /// Port `0x40001A4` settings for normal commands.
    ///
    /// Usually `0x00586000`.
    pub normal_command_settings: u32, // 0x060
    /// Port `0x40001A4` settings for KEY1 commands.
    ///
    /// Usually `0x001808F8`.
    pub normal_key1_settings: u32, // 0x064

    /// Icon/Title offset.
    ///
    /// `0x0000` for no icon/title.
    pub banner_offset: u32, // 0x068

    /// Secure area checksum.
    ///
    /// CRC-16 of `0x0020..=0x7FFF`.
    pub secure_area_crc16: u16, // 0x06C
    /// Secure area delay.
    ///
    /// In 131kHz units.
    ///
    /// - `0x051E` = 10ms
    /// - `0x0D7E` = 26ms
    pub secure_area_delay: u16, // 0x06E

    /// ARM9 autoload list hook RAM address?
    pub arm9_autoload: u32, // 0x070
    /// ARM7 autoload list hook RAM address?
    pub arm7_autoload: u32, // 0x074

    /// Secure area disable.
    ///
    /// By encrypted "NmMdOnly", usually zero.
    pub secure_area_disable: u64, // 0x078

    /// Total ROM size.
    ///
    /// Remaining/Unused bytes usually `0xFF` padded.
    pub rom_size: u32, // 0x080
    /// ROM header size.
    pub header_size: u32, // 0x084

    /// Unknown, some ROM offset or zero?
    unknown1: u32, // 0x088
    /// Reserved, zero filled.
    reserved2: [u8; 8], // 0x08C

    /// NAND end of ROM area.
    pub nand_rom_end: u16, // 0x094
    /// NAND start of RW area.
    ///
    /// Usually the same as [`nand_rom_end`].
    ///
    /// `0x0000` for none.
    ///
    /// [`nand_rom_end`]: #structfield.nand_rom_end
    pub nand_rw_start: u16, // 0x096

    /// Reserved, zero filled.
    reserved3: [u8; 40], // 0x098

    /// Nintendo logo.
    ///
    /// Compressed bitmap.
    pub nintendo_logo: [u8; 156], // 0x0C0
    /// Nintendo logo checksum.
    ///
    /// CRC-16 of `0x0C0..=0x15B`, usually fixed `0xCF56`.
    ///
    /// # Notes
    ///
    /// The BIOS only verifies that this is `0xCF56`, it does not verify the
    /// actual data of the logo. The logo data however is verified by the firmware.
    pub nintendo_logo_crc16: u16, // 0x15C
    /// Header checksum.
    ///
    /// CRC-16 of `0x000..=0x15D`, fixed `0xCF56`.
    pub header_crc16: u16, // 0x15E

    /// Debug ROM offset.
    ///
    /// `0x0000` for none.
    pub debug_rom_offset: u32, // 0x160
    /// Debug code size.
    ///
    /// Max `0x3BFE00` (3839.5KB). `0x0000` for none.
    pub debug_size: u32, // 0x164
    /// Debug RAM address.
    ///
    /// `0x2400000..=0x27BFE00`, or`0x0000` for none.
    pub debug_ram_address: u32, // 0x168

    /// Reserved, zero filled.
    ///
    /// Transferred and stored, but not used.
    reserved4: u32, // 0x16C
    /// Reserved, zero filled.
    ///
    /// Transferred, but not stored in RAM.
    reserved5: [u8; 144], // 0x170
}

// TODO: Static assert `size_of::<Header>() == 512`.
