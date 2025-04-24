use std::marker::PhantomData;

struct FourLetterWord([u8;4]);
impl FourLetterWord{
	const NULL:Self=Self([0;4]);
	fn new(s:&str)->Option<Self>{
		if s.len()==4{
			let mut arr=[0;4];
			arr.copy_from_slice(s.as_bytes());
			Some(Self(arr))
		}else{
			None
		}
	}
	fn as_str(&self)->&str{
		// SAFETY: the bytes were constructed with a valid string
		unsafe{std::str::from_utf8_unchecked(&self.0)}
	}
}
impl std::ops::Deref for FourLetterWord{
	type Target=str;
	fn deref(&self)->&Self::Target{
		self.as_str()
	}
}

struct LifetimeHost;

struct Words<'str>{
	len:usize,
	storage:[FourLetterWord;Words::CAP],
	_life:PhantomData<&'str LifetimeHost>,
}

impl<'str> Words<'str>{
	const CAP:usize=64;
	fn new<'a>(_host:&'a LifetimeHost)->Words<'a>
	{
		Words{
			len:0,
			storage:[FourLetterWord::NULL;Self::CAP],
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
			if word.as_str()==s{
				// SAFETY: who knows
				return unsafe{std::mem::transmute(word.as_str())};
			}
		}
		if self.len==Self::CAP{
			panic!("Words is full");
		}
		let new_word=FourLetterWord::new(s).unwrap();
		self.storage[self.len]=new_word;
		let word=self.storage[self.len].as_str();
		self.len+=1;
		// SAFETY: who knows
		unsafe{std::mem::transmute(word)}
	}
}

fn main(){
	let (a,b)={
		let host=LifetimeHost;
		let mut words=Words::new(&host);
		let a=words.intern("bruh");
		let b={
			// create a scoped reference to words
			let words=&words;
			// get a word
			words.get("bruh").unwrap()
			// drop the scope reference
		};
		(a,b)
	};
	// asjkdnaskjndjwkfaekjnfjklanw
	println!("{}",a==b);
}
