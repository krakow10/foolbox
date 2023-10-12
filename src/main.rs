fn main() {
	for radicand in 0..u32::MAX{
		//interpret bits as float
		let f_sqrt:u32=unsafe{(radicand as f32).sqrt().to_int_unchecked()};
		if f_sqrt.saturating_add(1)*f_sqrt.saturating_add(1)<radicand||radicand<f_sqrt.saturating_sub(1)*f_sqrt.saturating_sub(1){
			println!("rad:{} diff:{}",radicand,radicand as i64-(f_sqrt*f_sqrt) as i64);
		}
	}
}