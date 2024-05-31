use std::collections::HashMap;

fn main(){
	//big ole string
	let s = "the quick brown fox jumps over the lazy dog";
	//create a hashmap, h variable is mutable
	let mut h = HashMap::new();
	//loop over string characters
	for c in s.chars(){
		//handy rust tool "Entry"
		h.entry(c)
		//if the entry exists, add one to it
		.and_modify(|v|*v+=1)
		//otherwise, insert an entry for c and set it to 1
		.or_insert(1);
	}
	//print out the hashmap via debug print
	println!("{h:?}");
}