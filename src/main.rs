use hash_str::hstr;
use hash_str::{HashStr,HashStrMap};

#[allow(dead_code)]
#[derive(Debug)]
enum Error{
	Encode(rmp_serde::encode::Error),
	Decode(rmp_serde::decode::Error),
}

#[derive(serde::Serialize,serde::Deserialize)]
struct Outer<'a>{
	#[serde(borrow)]
	field:HashStrMap<'a,u32>,
}

fn main()->Result<(),Error>{
	let hstr1=hstr!("bruh");
	let hstr2=hstr!("yo");

	let mut field=HashStrMap::default();

	field.insert(hstr1, 1);
	field.insert(hstr2, 2);

	let outer=Outer{
		field,
	};

	let j=rmp_serde::encode::to_vec(&outer).map_err(Error::Encode)?;

	let outer2:Outer=rmp_serde::decode::from_slice(&j).map_err(Error::Decode)?;

	println!("hstr1={:?}",outer2.field.get(hstr1));
	println!("hstr2={:?}",outer2.field.get(hstr2));

	Ok(())
}
