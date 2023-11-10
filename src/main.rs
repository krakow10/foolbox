const V: [[i32; 3]; 8]=[
	[-1,-1, 1],
	[ 1,-1, 1],
	[ 1, 1, 1],
	[-1, 1, 1],
	[-1, 1,-1],
	[ 1, 1,-1],
	[ 1,-1,-1],
	[-1,-1,-1],
];
fn main() {
	let mut e=Vec::new();
	for i in 0..V.len(){
		for j in i..V.len(){
			let v0=V[i];
			let v1=V[j];
			let mut c=0;
			if v0[0]==v1[0]{
				c+=1;
			}
			if v0[1]==v1[1]{
				c+=1;
			}
			if v0[2]==v1[2]{
				c+=1;
			}
			if c==2{
				e.push([i,j]);
			}
		}
	}
	println!("{:?}",e);
}