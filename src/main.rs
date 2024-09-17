fn main() {
	let mut h=ahash::AHashSet::new();

	for a in 0..=(u16::MAX as u32){
		for b in 1..=a{
			let c=a*b;
			h.insert(c);
		}
	}

	println!("{}",h.len());
}
