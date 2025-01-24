use rand::Rng;

fn main(){
	let mut rng=rand::thread_rng();
	for _ in 0..100{
		println!("{:?}",f32::from_bits(rng.gen()));
	}
}
