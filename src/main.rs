fn main(){
	let a;
	let mut b = Box::new(0);// dynamic allocation
	a = 4;
	*b = 5;
	let c = a + *b;
	println!("{c}"); // prints 9
	//b is dropped when the scope ends
}