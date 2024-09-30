
fn main() {
	let a=fixed_wide::types::I32F32::ONE*50;
	let d=std::time::Instant::now();
	let r=a.sqrt();
	println!("t={:?}",d.elapsed());
	println!("ans={}",r);
	println!("ans={:#066b}",r.to_raw());
	//0b0000000000000000000000000000011100010010001100011000000000000111
}
