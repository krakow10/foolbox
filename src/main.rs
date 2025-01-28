pub const OBFUSCATION_NOISE_CYCLE_XOR:[u8;31]=[86,46,110,88,49,32,48,4,52,105,12,119,12,1,94,0,26,96,55,105,29,82,43,7,79,36,89,101,83,4,122];
fn reversible_obfuscate(offset:u64,buf:&mut [u8]){
	const LEN:u64=OBFUSCATION_NOISE_CYCLE_XOR.len() as u64;
	for (i,b) in buf.iter_mut().enumerate(){
		*b^=OBFUSCATION_NOISE_CYCLE_XOR[((offset+i as u64)%LEN) as usize];
	}
}
fn main(){
	let data=include_bytes!("../meshdata/4500696697_4.meshdata");
	let abc=data.len()-12;
	let mut datadata=data[abc..].to_vec();
	//reversible_obfuscate(abc as u64,&mut datadata);
	println!("{datadata:?}");
}
