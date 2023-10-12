fn main() {
	let mut n:i32=i32::MIN;
	println!("{:?}",n);
	n=n.wrapping_neg();
	println!("{:?}",n);
}