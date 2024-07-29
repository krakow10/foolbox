const fn bool_to_byte(b: bool) -> u8 {
    -(b as i8) as u8
}

fn main(){
	println!("{}",bool_to_byte(true));
	println!("{}",bool_to_byte(false));
}
