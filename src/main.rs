pub enum NormalId2{
	Right=1,
	Top=2,
	Back=3,
	Left=4,
	Bottom=5,
	Front=6,
}
fn main(){
	let whoa=NormalId2::Right as usize;
	println!("{whoa}");
}
