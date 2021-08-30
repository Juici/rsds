use common::str::Utf16;
use std::mem::{self, MaybeUninit};

/// NDS ROM icon/title.
///
/// The ROM offset is defined by [`banner_offset`] in [`Header`].
///
/// [`banner_offset`]: crate::nds::Header#structfield.banner_offset
/// [`Header`]: crate::nds::Header
///
/// # Title strings
///
/// The title is usually split into: a primary title, optional subtitle, and
/// manufacturer; each separated by a line feed (`0x000A`) and terminated by
/// `0x0000`.
///
/// eg. "Title", `0x000A`, "Subtitle", `0x000A`, "Manufacturer", `0x0000`.
///
/// # Sources
///
/// \[1\]: <https://problemkaputt.de/gbatek.htm#dscartridgeicontitle>
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct NdsBanner {
    /// Version.
    ///
    /// - `0x0001` = original
    /// - `0x0002` = with Chinese title
    /// - `0x0003` = with Chinese and Korean titles
    /// - `0x0103` = with Chinese and Korean titles, and animated DSi icon
    pub version: u16, // 0x0000
    /// Checksums (CRC16).
    ///
    /// - Entry 0 at `0x0002` = CRC16 for `0x0020..=0x083F` (all versions)
    /// - Entry 1 at `0x0004` = CRC16 for `0x0020..=0x093F` (version `0x0002` and above)
    /// - Entry 2 at `0x0006` = CRC16 for `0x0020..=0x0A3F` (version `0x0003` and above)
    /// - Entry 3 at `0x0006` = CRC16 for `0x1240..=0x23BF` (version `0x0103` and above)
    pub crc16: [u16; 4], // 0x0002
    /// Reserved, zero filled.
    reserved1: [u8; 22], // 0x000A
    /// Icon bitmap (32x32).
    ///
    /// 4x4 tiles, 4-bit depth.
    pub icon: [u8; 512], // 0x0020
    /// Icon palette.
    ///
    /// Range `0x0000..=0x7FFF`.
    ///
    /// Colour 0 is transparent, so the first palette entry is ignored.
    pub palette: [u16; 16], // 0x0220

    /// Japanese title.
    pub title_japanese: Utf16<128>, // 0x0240
    /// English title.
    pub title_english: Utf16<128>, // 0x0340
    /// French title.
    pub title_french: Utf16<128>, // 0x0440
    /// German title.
    pub title_german: Utf16<128>, // 0x0540
    /// Italian title.
    pub title_italian: Utf16<128>, // 0x0640
    /// Spanish title.
    pub title_spanish: Utf16<128>, // 0x0740
    /// Chinese title (version `0x0002` and above, or `0xFFFF` filled).
    pub title_chinese: Utf16<128>, // 0x0840
    /// Korean title (version `0x0003` and above, or `0xFFFF` filled).
    pub title_korean: Utf16<128>, // 0x0940

    /// Reserved, zero filled (probably for more titles).
    reserved2: [u8; 2048], // 0x0A40

    /// DSi icon animation bitmap.
    ///
    /// 4x4 tiles, 4-bit depth.
    pub dsi_icon: [[u8; 512]; 8], // 0x1240
    /// DSi icon animation palette.
    ///
    /// Range `0x0000..=0x7FFF`.
    ///
    /// Colour 0 is transparent, so the first palette entry is ignored.
    pub dsi_palette: [[u16; 16]; 8], // 0x2240
    /// DSi icon animation sequence.
    ///
    /// # Notes
    ///
    /// The sequence is represented by 16-bit tokens, in the following format:
    /// - `15` = flip vertically (`0` = no, `1` = yes)
    /// - `14` = flip horizontally (`0` = no, `1` = yes)
    /// - `13-11` = palette index (`0..=7`)
    /// - `10-8` = bitmap index (`0..=7`)
    /// - `7-0` = frame duration (`0x01..=0xFF`, in 60Hz units)
    ///
    /// `0x0000` indicates the end of the sequence.
    pub dsi_sequence: [u16; 64], // 0x2340
}

static_assert!(NdsBanner::SIZE == 9152);

impl NdsBanner {
    /// The size of a banner in bytes.
    pub const SIZE: usize = mem::size_of::<Self>();

    pub(crate) fn read(rom: &[u8], offset: usize) -> NdsBanner {
        assert!(rom.len() >= offset + NdsBanner::SIZE);

        let mut banner = MaybeUninit::uninit();

        let dst = banner.as_mut_ptr() as *mut u8;
        // SAFETY: `dst` is valid for writes of `BANNER_SIZE` bytes.
        //         `rom + offset` is valid for reads of `BANNER_SIZE` bytes.
        //         `dst` and `rom` are nonoverlapping.
        unsafe { dst.copy_from_nonoverlapping(rom.as_ptr().add(offset), NdsBanner::SIZE) };

        // SAFETY: `banner` is initialised with data copied from ROM.
        unsafe { banner.assume_init() }
    }
}
