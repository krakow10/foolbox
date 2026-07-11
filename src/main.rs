fn main(){
	let words=std::fs::read_to_string("/usr/share/dict/words").unwrap();
	for word in words.split_whitespace().filter(|word|word.contains("meow")){
		println!("{wabber}",wabber=word);
	}
}
