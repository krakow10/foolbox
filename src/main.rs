// A piece of data that does not implement the Copy trait
#[derive(Debug)]
struct NotCopy;

enum StackListGenerator{
	One,
	Two,
}

fn run_stack_list(variant:StackListGenerator){
	// How to write this code so that not_copy_owned is owned instead of a reference?
	let stack_list:&[NotCopy]=match variant{
		StackListGenerator::One=>&[NotCopy],
		StackListGenerator::Two=>&[NotCopy,NotCopy],
	};
	for not_copy_owned in stack_list{
		println!("List Value: {:?}",not_copy_owned);
	}
}

fn main(){
	run_stack_list(StackListGenerator::One);
	run_stack_list(StackListGenerator::Two);
}
