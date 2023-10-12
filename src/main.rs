fn gcd(mut a:u64,mut b:u64)->u64{
	while b!=0{
		(a,b)=(b,a.rem_euclid(b));
	};
	a
}
fn main() {
	println!("{}",gcd(6,12));
}