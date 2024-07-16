fn main(){
	//create a hashmap
	let mut h = std::collections::HashMap::new();

	//insert a key-value pair
	h.insert("key1","value1");

	//access the value using the key
	println!("{}",h["key1"]);// -> value1
}