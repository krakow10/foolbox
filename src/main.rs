const fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        (a, b) = (b, a.rem_euclid(b));
    }
    a
}

fn main(){
	dbg!(gcd(0,1));
	dbg!(gcd(1,0));
	dbg!(gcd(1,1));
	dbg!(gcd(999999999,9));
}
