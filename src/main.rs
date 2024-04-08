
fn main(){
	let mut s=String::from("hello");
	let b=unsafe{s.as_bytes_mut()};
  //if(97..123).contains(&b[0]){b[0]-=32;}
  //if 96<b[0]&&b[0]<123{b[0]-=32}
  //if let 97..=122=b[0]{b[0]^=32}
	b[0].make_ascii_uppercase();
	println!("{}",s);
}