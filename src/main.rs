fn main() {
	let not_0_01=0.01f32;
	let mut s=0.0;
	for _ in 0..100{
		s+=not_0_01;
	}
	println!("{:.32}",s);
}
