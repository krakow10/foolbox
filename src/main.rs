use hash_str::hstr;
use hash_str::HashStr;

#[allow(dead_code)]
#[derive(Debug)]
enum Error{
	Encode(rmp_serde::encode::Error),
	Decode(rmp_serde::decode::Error),
}

fn main()->Result<(),Error>{
	let hstr_static=hstr!("bruh");

	let j=rmp_serde::encode::to_vec(hstr_static).map_err(Error::Encode)?;

	let h2:&HashStr=rmp_serde::decode::from_slice(&j).map_err(Error::Decode)?;

	assert_eq!(hstr_static,h2);

	Ok(())
}
