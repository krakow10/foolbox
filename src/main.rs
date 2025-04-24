use std::marker::PhantomData;

struct LifetimeHost;

// grow-only list of words
struct Words<'str>{
	len:usize,
	storage:[String;Words::CAP],
	_life:PhantomData<&'str LifetimeHost>,
}

// SAFETY: caller must ensure that 'short lifetime is actualy valid for 'long
unsafe fn extend_lifetime<'short,'long,T:?Sized>(short:&'short T)->&'long T{
	std::mem::transmute(short)
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
				// SAFETY: I promise not to mutate the strings in the implementation of Words
				return Some(unsafe{extend_lifetime(word.as_str())});
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
		self.storage[self.len]=s.to_owned();
		let word=self.storage[self.len].as_str();
		self.len+=1;
		// SAFETY: I promise not to mutate the strings in the implementation of Words
		unsafe{extend_lifetime(word)}
	}
}

fn main(){
	let lifetime_host=LifetimeHost;
	let mut words=Words::new(&lifetime_host);

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
	// drop(lifetime_host);
	println!("{}",a==b);
}
