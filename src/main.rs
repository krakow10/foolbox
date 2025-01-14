// safety: I have no idea what I'm doing and this is potentially a recipe for disaster
fn replace_with<T>(dest:&mut T,f:impl Fn(T)->T){
	unsafe{
		let peek=core::ptr::read(dest);
		let result=f(peek);
		core::ptr::write(dest,result);
	}
}

// Some type that does not implement the Copy trait
struct NotCopy;

// The enum which needs to change variants
enum Propagate{
	StageOne,
	StageTwo(NotCopy),
	StageThree(NotCopy),
	StageFour,
}

// method to replace self with another variant
impl Propagate{
	fn transform(self)->Self{
		match self{
			Propagate::StageOne=>Propagate::StageTwo(NotCopy),
			Propagate::StageTwo(not_copy)=>Propagate::StageThree(not_copy),
			Propagate::StageThree(_not_copy)=>Propagate::StageFour,
			Propagate::StageFour=>Propagate::StageOne,
		}
	}
	fn propagate(&mut self){
		replace_with(self,Self::transform);
	}
}

fn main() {
	let mut e=Propagate::StageOne;

	e.propagate();
	e.propagate();
	e.propagate();
	e.propagate();
}
