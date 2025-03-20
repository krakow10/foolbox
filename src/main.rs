fn main(){
	const GON:i32=3;
	const POINTS:[[i32;2];24]=const{
		let mut points=[[0;2];24];
		let mut x=-GON;
		while x<GON{
			points[(x+GON) as usize]=[x,GON];
			points[(x+GON+2*2*GON) as usize]=[-x,-GON];
			x+=1;
		}
		let mut y=-GON;
		while y<GON{
			points[(y+GON+1*2*GON) as usize]=[GON,-y];
			points[(y+GON+3*2*GON) as usize]=[-GON,y];
			y+=1;
		}
		points
	};
	let mut s=format!("{:?}",POINTS);
	s=s.replace('[',"{");
	s=s.replace(']',"}");
	println!("{s}");
}
