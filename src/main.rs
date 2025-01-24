fn main(){
	let data=include_bytes!("../meshdata/385416572.meshdata");
	let first=&data[0..23];
	println!("first {first:?}");
	let mut count=0;
	let mut im=image::ImageBuffer::<image::Luma<u8>,_>::new(31,193);
	for (y,chunk) in data[23..].chunks_exact(31).enumerate(){
		count+=1;
		for (x,b) in chunk.iter().enumerate(){
			im.put_pixel(x as u32,y as u32,image::Luma([*b]));
			print!("{b:3.},");
		}
		println!();
	}
	println!("chunks={count}");
	let mut file=std::fs::File::create("meshdata.png").unwrap();
	im.write_to(&mut file,image::ImageFormat::Png).unwrap();
}
