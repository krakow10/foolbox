type Mesh=i32;
struct MeshId(u32);
struct MeshVec(Vec<Mesh>);

impl std::ops::Index<MeshId> for MeshVec{
	type Output=Mesh;
	fn index(&self, index: MeshId) -> &Self::Output {
		&self.0[index.0 as usize]
	}
}
impl std::ops::Deref for MeshVec{
	type Target = Vec<Mesh>;
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

fn main() {
	let ilist=vec![1,2,3];
	//a
	let m=MeshVec(ilist.clone());
	let a=m[MeshId(1)];
	println!("a={}",a);
	//b
	let b=ilist[1];
	println!("b={}",b);

	//iter
	for v in m.iter(){
		println!("v={}",v);
	}
	let c=std::borrow::Cow::Borrowed(&ilist);
	for v in c.iter(){
		println!("v={}",v);
	}
}