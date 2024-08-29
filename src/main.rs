
fn main() {
	let a=fixed_wide::types::I32F32::ONE*2;
	let d=std::time::Instant::now();
	let r=a.sqrt();
	println!("t={:?}",d.elapsed());
	println!("ans={:?}",r);
}
