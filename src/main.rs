fn main(){
	//these values have the type &str which is called "string slice"
	let words = ["cat","bat","rat","mat","that","chair","sat","me","at"];
	let _at: Vec<&str> = 
	words
	//convert the words array into an iterator, consuming it
	.into_iter()
	//filter out values according to a function that returns true or false
	.filter(|&word|{
		if word.len()!=3{
			//filter out words not length 3
			return false;
		}
		match &word[1..3]{ //this operation is "slice"
			"at"=>true,
			//filter out words that don't have "at" at character positions 1..3 (non-inclusive range)
			_=>false,
		}
	})
	//collect the iterator into a Vec<&str>
	.collect();
	println!("filtered words {_at:?}"); // :? means debug print
}