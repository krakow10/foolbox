fn main(){
	let data=include_bytes!("../meshdata/394453730.meshdata");
	let noise=[86,46,110,88,49,32,48,4,52,105,12,119,12,1,94,0,26,96,55,105,29,82,43,7,79,36,89,101,83,4,122];
	let actual_data:Vec<_>=data.iter().enumerate().map(|(i,b)|b^noise[i%noise.len()]).collect();
	std::fs::write("decoded_meshdata_394453730.meshdata",actual_data).unwrap();
}
