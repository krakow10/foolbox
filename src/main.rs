use std::collections::HashMap;

fn main() {
	let mut h=HashMap::new();

	let r=0..10;
	for i in r{
		h.entry(i%3)
			.and_modify(|a|(*a)+=1)
			.or_insert(0);
	}

	let all_mut:Vec<_>=h.iter_mut().collect();

	println!("{:?}",h);
}
