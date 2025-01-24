fn main(){
	let data=include_bytes!("../meshdata/385416572.meshdata");
	let offset=0;
	for columns in 1..256{
		let rows=data.len()/columns;
		let first=&data[0..offset];
		println!("first {first:?}");
		let mut im=image::ImageBuffer::<image::Luma<u8>,_>::new(columns as u32,rows as u32);
		for (y,chunk) in data[offset..].chunks_exact(columns).enumerate(){
			for (x,b) in chunk.iter().enumerate(){
				im.put_pixel(x as u32,y as u32,image::Luma([*b]));
			}
		}
		let mut file=std::fs::File::create(format!("meshdata{columns}.png")).unwrap();
		im.write_to(&mut file,image::ImageFormat::Png).unwrap();
	}
}
