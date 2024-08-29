
fn main() {
	let a=fixed_wide::types::I256F256::ONE*2;
	let d=std::time::Instant::now();
	let r=a.sqrt();
	println!("t={:?}",d.elapsed());
	println!("ans={:?}",r);
}
