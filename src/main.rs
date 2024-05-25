fn main(){
	let t = [9,3,7,3,7,32,76,8,3,21];
	let smallest = t.iter().fold(std::i32::MAX,|s,&v|if v<s{v}else{s});
	println!("smallest {smallest}");
}