const JUMP_TIME:f32=0.715588;//78/109
fn main() {
	let (i,f)=(1..200).map(|i|(i,JUMP_TIME*i as f32)).min_by_key(|&(_,f0)|{
		let fr=f0.fract();
		(fr.min(1.0-fr)*(1u64<<32) as f32) as u32
	}).unwrap();
	println!("min index: {i}\nmin value:{f}");
}
