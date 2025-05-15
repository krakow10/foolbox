fn main(){
	let bo = Box::new(String::from("test"));
	let string: String = *bo;
	println!("{string}");
	println!("{bo}");
}
