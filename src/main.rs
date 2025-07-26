#[repr(C)]
struct BitHeader {
    /// 0h: BIT Header Identifier (BMP=0x7FFF/BIT=0xB8FF)
    id: u16,
    /// 2h: BIT Header Signature ("BIT\0")
    signature: [u8; 4],
    /// 6h: Binary Coded Decimal Version, ex: 0x0100 is 1.00.
    bcd_version: u16,
    /// 8h: Size of BIT Header (in bytes)
    header_size: u8,
    /// 9h: Size of BIT Tokens (in bytes)
    token_size: u8,
    /// 10h: Number of token entries that follow
    token_entries: u8,
    /// 11h: BIT Header Checksum
    checksum: u8,
}
fn main() {
    let a = PmuLookupTableHeader::new(&[1, 2, 3, 4]);
    dbg!(a);
}
