fn main() {
	let a=[false,true];
	for x in a{
		for y in a{
			assert_eq!(x^y,x!=y);
		}
	}
}
