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
		if let Some(ustr)=self.get(s){
			return ustr;
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

	// borrow Words mutably
	let a=words.intern("bruh");
	// drop mutable borrow and borrow immutably
	let b=words.get("bruh").unwrap();
	// compare both references; this is impossible when
	// the lifetimes of a and b are derived from
	// the borrows in .get and .intern
	// e.g.
	// fn    get<'a>(&'a     self,s:&str)->Option<&'a str>{
	// fn intern<'a>(&'a mut self,s:&str)->       &'a str {
	// instead of the lifetime of the underlying data 'str
	println!("{}",a==b);

	// with a correct implementation,
	// dropping words here should introduce a compile error
	drop(words);
	println!("{}",a==b);

	// dropping LifetimeHost gives the desired compile error
	// drop(host);
	println!("{}",a==b);
}
