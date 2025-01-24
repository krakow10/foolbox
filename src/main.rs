fn main(){
	let data=include_bytes!("../385416572.meshdata");
	let first=&data[0..23];
	println!("first {first:?}");
	let mut it=data[23..].chunks_exact(31);
	let mut count=0;
	for chunk in &mut it{
		count+=1;
		for b in chunk{
			print!("{b:3.},");
		}
		println!();
	}
	println!("chunks={count}");
}
