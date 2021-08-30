use std::fmt;
use std::mem::{self, MaybeUninit};

use common::str::Ascii;
use common::util::FileSize;

// TODO: Add proper support for DSi headers.

/// NDS ROM header.
///
/// Loaded from `0x00` in ROM to `0x27FFE00` on power-up.
///
/// # Sources
///
/// \[1\]: <https://problemkaputt.de/gbatek.htm#dscartridgeheader>
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct NdsHeader {
    /// Game title.
    ///
    /// Uppercase ASCII, padded with `0x00`.
    pub game_title: Ascii<12>, // 0x000
    /// Game code.
    ///
    /// Uppercase ASCII, `NTR-{code}`.
    pub game_code: Ascii<4>, // 0x00C
    /// Maker code.
    ///
    /// Uppercase ASCII, eg. `01` is Nintendo.
    pub maker_code: Ascii<2>, // 0x010
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
    pub key1_command_settings: u32, // 0x064

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
    ///
    /// In `0x20000` byte units (`0x80000` on DSi). `0x0000` for none.
    ///
    /// Usually the same as [`nand_rw_start`].
    ///
    /// [`nand_rw_start`]: #structfield.nand_rw_start
    pub nand_rom_end: u16, // 0x094
    /// NAND start of RW area.
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

static_assert!(NdsHeader::SIZE == 512);

impl NdsHeader {
    /// The size of a header in bytes.
    pub const SIZE: usize = mem::size_of::<Self>();

    pub(crate) fn read(rom: &[u8]) -> NdsHeader {
        assert!(rom.len() >= NdsHeader::SIZE);

        let mut header = MaybeUninit::uninit();

        let dst = header.as_mut_ptr() as *mut u8;
        // SAFETY: `dst` is valid for writes of `HEADER_SIZE` bytes.
        //         `rom` is valid for reads of `HEADER_SIZE` bytes.
        //         `dst` and `rom` are nonoverlapping.
        unsafe { dst.copy_from_nonoverlapping(rom.as_ptr(), NdsHeader::SIZE) };

        // SAFETY: `header` is initialised with data copied from ROM.
        unsafe { header.assume_init() }
    }

    /// Returns the device capacity in bytes.
    pub fn device_capacity_bytes(&self) -> usize {
        (128 * 1024) << self.device_capacity
    }

    /// Computes the Nintendo logo checksum.
    pub fn compute_logo_crc16(&self) -> u16 {
        common::util::crc16(&self.nintendo_logo)
    }

    /// Computes the header checksum.
    pub fn compute_header_crc16(&self) -> u16 {
        let ptr = self as *const NdsHeader as *const u8;
        // Header CRC16 is computed over `0x000..0x15E`.
        let bytes = unsafe { std::slice::from_raw_parts(ptr, 0x15E) };
        common::util::crc16(bytes)
    }
}

impl fmt::Display for NdsHeader {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        macro_rules! none_if_0 {
            ($value:expr) => {
                if $value == 0 { " (None)" } else { "" }
            };
        }

        writeln!(f, "0x000  Game title                          {}", self.game_title)?;
        writeln!(f, "0x00C  Game code                           {}", self.game_code)?;
        writeln!(f, "0x010  Maker code                          {}", self.maker_code)?;
        writeln!(f, "0x012  Unit code                           {:#04X}", self.unit_code)?;
        writeln!(f, "0x013  Encryption seed select              {:#04X}", self.encryption_seed_select)?;
        writeln!(f, "0x014  Device capacity                     {:#04X} ({})", self.device_capacity, FileSize(self.device_capacity_bytes()))?;
        writeln!(f, "0x015  (8 bytes reserved)")?;
        writeln!(f, "0x01D  NDS region                          {:#04X}", self.nds_region)?;
        writeln!(f, "0x01E  ROM version                         {:#04X}", self.rom_version)?;
        writeln!(f, "0x01F  Autostart                           {:#04X}", self.autostart)?;

        writeln!(f, "0x020  ARM9 ROM offset                     {:#X}", self.arm9_rom_offset)?;
        writeln!(f, "0x024  ARM9 entry address                  {:#X}", self.arm9_entry_address)?;
        writeln!(f, "0x028  ARM9 RAM address                    {:#X}", self.arm9_ram_address)?;
        writeln!(f, "0x02C  ARM9 code size                      {:#X}", self.arm9_size)?;

        writeln!(f, "0x030  ARM7 ROM offset                     {:#X}", self.arm7_rom_offset)?;
        writeln!(f, "0x034  ARM7 entry address                  {:#X}", self.arm7_entry_address)?;
        writeln!(f, "0x038  ARM7 RAM address                    {:#X}", self.arm7_ram_address)?;
        writeln!(f, "0x03C  ARM7 code size                      {:#X}", self.arm7_size)?;

        writeln!(f, "0x040  File name table (FNT) offset        {:#X}", self.fnt_offset)?;
        writeln!(f, "0x044  File name table (FNT) size          {:#X}", self.fnt_size)?;
        writeln!(f, "0x048  File allocation table (FAT) offset  {:#X}", self.fat_offset)?;
        writeln!(f, "0x04C  File allocation table (FAT) size    {:#X}", self.fat_size)?;

        writeln!(f, "0x050  ARM9 overlay offset                 {:#X}", self.arm9_overlay_offset)?;
        writeln!(f, "0x054  ARM9 overlay size                   {:#X}", self.arm9_overlay_size)?;
        writeln!(f, "0x058  ARM7 overlay offset                 {:#X}", self.arm7_overlay_offset)?;
        writeln!(f, "0x05C  ARM7 overlay size                   {:#X}", self.arm7_overlay_size)?;

        writeln!(f, "0x060  Normal commands settings            {:#010X}", self.normal_command_settings)?;
        writeln!(f, "0x064  KEY1 commands settings              {:#010X}", self.key1_command_settings)?;

        writeln!(f, "0x068  Banner offset                       {:#X}{}", self.banner_offset, none_if_0!(self.banner_offset))?;

        writeln!(f, "0x06C  Secure area checksum                {:#06X}", self.secure_area_crc16)?;
        let delay_ms = self.secure_area_delay as f64 / 131.0;
        writeln!(f, "0x06E  Secure area delay                   {:#06X} ({:.0} ms)", self.secure_area_delay, delay_ms)?;

        writeln!(f, "0x070  ARM9 autoload hook RAM address?     {:#X}", self.arm9_autoload)?;
        writeln!(f, "0x074  ARM7 autoload hook RAM address?     {:#X}", self.arm7_autoload)?;

        writeln!(f, "0x078  Secure area disable                 {:#018X}", self.secure_area_disable)?;

        writeln!(f, "0x080  ROM size                            {:#X}", self.rom_size)?;
        writeln!(f, "0x084  ROM header size                     {:#X}", self.header_size)?;

        writeln!(f, "0x088  (4 bytes unknown)")?;
        writeln!(f, "0x08C  (8 bytes reserved)")?;

        writeln!(f, "0x094  NAND end of ROM area                {:#06X}", self.nand_rom_end)?;
        writeln!(f, "0x096  NAND start of RW area               {:#06X}", self.nand_rw_start)?;

        writeln!(f, "0x098  (40 bytes reserved)")?;

        struct Crc16Validate {
            validation: u16,
            computed: u16,
        }

        impl fmt::Display for Crc16Validate {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                if self.validation == self.computed {
                    write!(f, "OK")
                } else {
                    write!(f, "INVALID {:#06X}", self.computed)
                }
            }
        }

        let logo_crc16 = Crc16Validate {
            validation: self.nintendo_logo_crc16,
            computed: self.compute_logo_crc16(),
        };
        let header_crc16 = Crc16Validate {
            validation: self.header_crc16,
            computed: self.compute_header_crc16(),
        };

        writeln!(f, "0x0C0  Nintendo logo (156 bytes)")?;
        writeln!(f, "0x15C  Nintendo logo checksum              {:#06X} ({})", self.nintendo_logo_crc16, logo_crc16)?;
        writeln!(f, "0x15E  Header checksum                     {:#06X} ({})", self.header_crc16, header_crc16)?;

        writeln!(f, "0x160  Debug ROM offset                    {:#X}{}", self.debug_rom_offset, none_if_0!(self.debug_rom_offset))?;
        writeln!(f, "0x164  Debug code size                     {:#X}{}", self.debug_size, none_if_0!(self.debug_size))?;
        writeln!(f, "0x168  Debug RAM address                   {:#X}{}", self.debug_ram_address, none_if_0!(self.debug_ram_address))?;

        writeln!(f, "0x16C  (4 bytes reserved)")?;
        write!(f, "0x170  (144 bytes reserved)")?;

        Ok(())
    }
}
