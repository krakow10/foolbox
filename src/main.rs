fn test<'a,S:Into<&'a str>>(s:S)->&'a str{
	s.into()
}
fn main(){
	let s=String::from("test");
	let t=test(s.as_str());
}
