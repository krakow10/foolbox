use binrw::BinWrite;

#[binrw::binrw]
#[brw(little)]
pub struct Header2{
	sizeof_header:u16,//size of this struct...
	sizeof_vertex:u8,
	sizeof_face:u8,
}

fn main() {
	let h=Header2{
		sizeof_header:12,
		sizeof_vertex:40,
		sizeof_face:12,
	};
	let mut buf=Vec::new();
	let mut c=std::io::Cursor::new(&mut buf);
	h.write_le(&mut c).unwrap();
	println!("{:?}",c);
	println!("hi");
}