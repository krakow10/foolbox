fn main() {
	for _ in 0..1000 {
		let start=std::time::Instant::now();
		std::thread::spawn(move || {
			println!("{}",start.elapsed().as_nanos());
		});
		//wait for the result to be printed to console
		std::thread::sleep(std::time::Duration::from_millis(100));
	}
}