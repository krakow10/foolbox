fn main(){
	let mut a=1;
	let mut b=2;
	std::mem::swap(&mut a,&mut b);
	println!("a={} b={}",a,b);
}