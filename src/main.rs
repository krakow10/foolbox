struct Circle{
	radius:f32,
}
impl Circle{
	fn new(radius:f32)->Self{
		Self{
			radius
		}
	}
	fn get_area(&self)->f32{
		std::f32::consts::PI*self.radius*self.radius
	}
	fn get_circumference(&self)->f32{
		std::f32::consts::TAU*self.radius
	}
}
fn main(){
	let c=Circle::new(6.0);
	println!("The radius is {}",c.radius);
	println!("The area is {}",c.get_area());
	println!("The circumference is {}",c.get_circumference());
}