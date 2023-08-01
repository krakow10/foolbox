fn main() {
	let a:u64 = 0x00000000_90000000;
	let b:u64 = 0x00000010_00000000;
	let c:u64 = (((a as u128)*(b as u128))>>64) as u64;
	println!("{:?}",c);
}