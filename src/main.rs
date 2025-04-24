use std::marker::PhantomData;

struct LifetimeHost;

// grow-only list of words
struct Words<'str>{
	len:usize,
	storage:[String;Words::CAP],
	_life:PhantomData<&'str LifetimeHost>,
}

impl<'str> Words<'str>{
	const CAP:usize=64;
	fn new<'a>(_host:&'a LifetimeHost)->Words<'a>
	{
		Words{
			len:0,
			storage:std::array::from_fn(|_|String::new()),
			_life:PhantomData,
		}
	}
	fn get(&self,s:&str)->Option<&'str str>{
		for word in &self.storage{
			if word.as_str()==s{
				// SAFETY: who knows
				return Some(unsafe{std::mem::transmute(word.as_str())});
			}
		}
		None
	}
	fn intern(&mut self,s:&str)->&'str str{
		for word in &self.storage{
			if word==s{
				// SAFETY: who knows
				return unsafe{std::mem::transmute(word.as_str())};
			}
		}
		if self.len==Self::CAP{
			panic!("Words is full");
		}
		let new_word=s.to_owned();
		self.storage[self.len]=new_word;
		let word=self.storage[self.len].as_str();
		self.len+=1;
		// SAFETY: who knows
		unsafe{std::mem::transmute(word)}
	}
}

fn main(){
	let host=LifetimeHost;
	let mut words=Words::new(&host);
	// with a correct implementation, the following code should be valid:
	let a=words.intern("bruh");
	let b={
		// create a scoped reference to words
		let words=&words;
		// get a word
		words.get("bruh").unwrap()
		// drop the scope reference
	};
	println!("{}",a==b);

	// with a correct implementation,
	// dropping words here should introduce a compile error
	drop(words);
	println!("{}",a==b);
}
