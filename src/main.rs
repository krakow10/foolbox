fn s(s:usize)->isize{
	(s as isize).signum()
}
fn main() {
	let a=1isize;
	let v=vec![1];
	println!("{}",v[a]);
}