fn main() {
	// the value, a "static" meaning it
	// exists for the entire lifetime of the program, not just this function call,
	// and always refers to one singular global value.
	static mut VALUE:&mut i64=&mut 0;

	// how many cpu cores do we have to work with?
	let parallelism = std::thread::available_parallelism().unwrap().get();

	println!("threads: {parallelism}");

	// spawn threads attempting to add one
	for _ in 0..parallelism/2{
		std::thread::spawn(||{
			loop{
				// this is undefined behaviour!
				unsafe{*VALUE+=1}
			}
		});
	}
	// spawn threads attempting to subtract one
	for _ in 0..parallelism/2{
		std::thread::spawn(||{
			loop{
				// this is undefined behaviour!
				unsafe{*VALUE-=1}
			}
		});
	}

	// monitor the chaos every 100ms
	loop{
		println!("{}",unsafe{*VALUE});
		std::thread::sleep(std::time::Duration::from_millis(100));
	}
}
