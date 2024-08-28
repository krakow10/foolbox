fn gcd(mut a:f64,mut b:f64)->f64{
	while b!=0f64{
		(a,b)=(b,a.rem_euclid(b));
	};
	a
}

fn main() {
	let a=0.1298374568127365872345;
	let b=897231895434.23456234859723495;
	let c=gcd(a,b);
	println!("{}",c);
	println!("{}",a/c);
	println!("{}",b/c)
}
