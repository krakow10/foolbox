fn capitalize(s:&str)->String{
	if s.len()==0{
		return "".to_owned();
	}
	let a=s[0..1].to_uppercase();
	let b=a+&s[1..];
	b
}
fn main(){
	println!("{}",capitalize(""));
}