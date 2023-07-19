use std::time::Instant;

fn main() {
    let t0=Instant::now();
    println!("123");
    let t1=Instant::now();
    println!("{:?}",t1.duration_since(t0));
}
