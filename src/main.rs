use std::mem::offset_of;

#[derive(Debug)]
struct EINVAL;

#[derive(Debug)]
struct PmuLookupTableHeader {
    version: u8,
    header_len: u8,
    entry_len: u8,
    entry_count: u8,
}

impl PmuLookupTableHeader {
    // TODO[TRSM]: use FromBytes::from_bytes when it becomes available.
    fn new(data: &[u8]) -> Result<Self,EINVAL> {
        if data.len() < core::mem::size_of::<Self>() {
            return Err(EINVAL);
        }

        Ok(PmuLookupTableHeader {
            version: data[const{offset_of!(PmuLookupTableHeader,version)}],
            header_len: data[const{offset_of!(PmuLookupTableHeader,header_len)}],
            entry_len: data[const{offset_of!(PmuLookupTableHeader,entry_len)}],
            entry_count: data[const{offset_of!(PmuLookupTableHeader,entry_count)}],
        })
    }
}
fn main(){
	let a=PmuLookupTableHeader::new(&[1,2,3,4]);
	dbg!(a);
}
