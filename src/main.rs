fn main() {
	let mut h=ahash::AHashSet::new();

	for a in 1..=u16::MAX{
		for b in 1..=a{
			let c=(a as u32)*(b as u32);
			h.insert(c);
		}
	}

	println!("{}",h.len());
}
