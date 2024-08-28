const fn gcd(mut a:u128,mut b:u128)->u128{
	while b!=0{
		(a,b)=(b,a.rem_euclid(b));
	};
	a
}

fn main() {
	const div:u128=10000000000000000000000u128;
	const a:u128=1298374568127365872345u128;
	const b:u128=8972318954342345623485972349500000u128;
	const c:u128=gcd(a,b);
	println!("{}",c);
	println!("{}",a/c);
	println!("{}",b/c);
	println!("{}",c as f64/div as f64);
}
