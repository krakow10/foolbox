use hash_str::hstr;
use hash_str::HashStr;

#[allow(dead_code)]
#[derive(Debug)]
enum Error{
	Encode(rmp_serde::encode::Error),
	Decode(rmp_serde::decode::Error),
}

#[derive(serde::Serialize,serde::Deserialize)]
struct Outer<'a>{
	#[serde(borrow)]
	field:&'a HashStr,
}

fn main()->Result<(),Error>{
	let hstr_static=hstr!("bruh");

	let outer=Outer{
		field:hstr_static,
	};

	let j=rmp_serde::encode::to_vec(&outer).map_err(Error::Encode)?;

	let h2:Outer=rmp_serde::decode::from_slice(&j).map_err(Error::Decode)?;

	println!("before={}",hstr_static);
	println!("after={}",h2.field);

	assert_eq!(hstr_static,h2.field);

	Ok(())
}
