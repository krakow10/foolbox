fn main() {
    let mut lines_list = vec!["line1", "line2", "line3", "line4"];
    let mut it = lines_list.iter();

	it.next();
	std::mem::drop(it);
	lines_list.pop();
	it=lines_list.iter();

    // iterator still exists and can be used
    for line in it {
        println!("{}", line);
    }
}
