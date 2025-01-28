fn main(){
	let data=include_bytes!("../meshdata/15124417947_5.meshdata");
	for columns in 4..84{
		let rows=data.len()/columns;
		let mut im=image::ImageBuffer::<image::Luma<u8>,_>::new(columns as u32,rows as u32);
		for (y,chunk) in data.chunks_exact(columns).enumerate(){
			for (x,b) in chunk.iter().enumerate(){
				im.put_pixel(x as u32,y as u32,image::Luma([*b]));
			}
		}
		let mut file=std::fs::File::create(format!("meshdata{columns}.png")).unwrap();
		im.write_to(&mut file,image::ImageFormat::Png).unwrap();
	}
}
