
fn main(){
	let a:Vec<u8>=(0..16).collect();
	let b=b"0123456789abcdef";
	for i in 0..16{
		println!("{} lhs={:08b} rhs={:08b}",char::from(b[i]),a[i],b[i]);
	}
}
