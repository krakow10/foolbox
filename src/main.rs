use hash_str::hstr;
use hash_str::HashStr;
use hash_str::HashStrHost as LifetimeHost;
use hash_str::HashStrCache as Words;

fn main(){
	let bruh_static=hstr!("bruh");
	let bruh_runtime=&*HashStr::anonymous("bruh".to_owned());
	// TODO: implement the functionality of LifetimeHost directly into Words
	let lifetime_host=LifetimeHost::new();
	let mut words=Words::new();

	// borrow Words mutably
	let a=words.intern(&lifetime_host,"bruh");
	// drop mutable borrow and borrow immutably
	let b=words.get("bruh").unwrap();
	let c=words.get(bruh_static).unwrap();
	let d=words.get(bruh_runtime).unwrap();
	// compare both references; this is impossible when
	// the lifetimes of a and b are derived from
	// the borrows in .get and .intern
	// e.g.
	// fn    get<'a>(&'a     self,s:&str)->Option<&'a str>{
	// fn intern<'a>(&'a mut self,s:&str)->       &'a str {
	// instead of the lifetime of the underlying data 'str
	println!("{}",a==b);
	println!("{}",a==c);
	println!("{}",a==d);
	println!("{}",a==bruh_static);
	println!("{}",a==bruh_runtime);

	// with a correct implementation,
	// dropping words here should introduce a compile error
	drop(words);
	println!("{}",a==b);

	// dropping LifetimeHost gives the desired compile error
	// drop(lifetime_host);
	println!("{}",a==b);
}
