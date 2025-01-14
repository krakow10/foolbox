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
	fn propagate(&mut self){
		*self=match *self{
			Propagate::StageOne=>Propagate::StageTwo(NotCopy),
			Propagate::StageTwo(not_copy)=>Propagate::StageThree(not_copy),
			Propagate::StageThree(_not_copy)=>Propagate::StageFour,
			Propagate::StageFour=>Propagate::StageOne,
		}
	}
}

fn main() {
	let mut e=Propagate::StageOne;

	e.propagate();
	e.propagate();
	e.propagate();
	e.propagate();
}
