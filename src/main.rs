fn main(){
	let data=include_bytes!("../meshdata/385416572.meshdata");
	let noise=[86,46,110,88,49,32,48,4,52,105,12,119,12,1,94,0,26,96,55,105,29,82,43,7,79,36,89,101,83,4,122];
	let columns=84;
	let rows=data.len()/columns;
	let mut c=0;
	let mut im=image::ImageBuffer::<image::Luma<u8>,_>::new(columns as u32,rows as u32);
	for (y,chunk) in data.chunks_exact(columns).enumerate(){
		for (x,b) in chunk.iter().enumerate(){
			im.put_pixel(x as u32,y as u32,image::Luma([-((*b!=noise[c%noise.len()]) as i8) as u8]));
			c+=1;
		}
	}
	let mut file=std::fs::File::create(format!("meshdata{columns}.png")).unwrap();
	im.write_to(&mut file,image::ImageFormat::Png).unwrap();
}
