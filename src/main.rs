fn main() {
	let result;
	{
		let numba=2;

		result=&numba;
		//numba dies
	}
	println!("{:?}",result);
}
