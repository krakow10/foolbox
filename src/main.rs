
fn main(){
	let buf=&[0,1,2,3,4];

	if buf.len()<5{
		panic!();
	}

	let slice=&buf[2..6];
	println!("{slice:?}");
}
