use rayon::iter::{IntoParallelIterator, ParallelIterator};

fn main() {
	(0..u32::MAX).into_par_iter().for_each(|bits|{
		let a=f32::from_bits(bits);
		let b:Result<fixed_wide::types::I32F32,_>=a.try_into();
		if b.is_err()&&bits%1351531==0{
			println!("bits={bits}");
		}
	});
}
