// Array-backed vector with named field access

#[derive(Clone,Copy)]
struct Array<const N:usize>([f32;N]);
impl<const N:usize> Array<N>{
	const fn new(value:[f32;N])->Self{
		Self(value)
	}
	fn dot(self,other:Self)->f32{
		self.0.into_iter().zip(other.0).map(|(a,b)|a*b).sum()
	}
}

#[derive(Clone,Copy)]
struct Vector3{
	x:f32,
	y:f32,
	z:f32,
}

impl std::ops::Deref for Array<3>{
	type Target=Vector3;
	fn deref(&self)->&Self::Target{
		unsafe{std::mem::transmute(self)}
	}
}
impl std::ops::DerefMut for Array<3>{
	fn deref_mut(&mut self)->&mut Self::Target{
		unsafe{std::mem::transmute(self)}
	}
}

// Array-backed Matrix with named field access

// Mat<3,Vec<2>>.dot(M)
// M.x_axis
// 	self.map(|axis|
// 		tr.map(|trax|
// 			axis.dot(trax)
// 		).to_vector()
// 	)
#[derive(Clone,Copy)]
struct Array2d<const X:usize,const Y:usize>([[f32;Y];X]);
impl<const X:usize,const Y:usize> Array2d<X,Y>{
	const fn new(value:[[f32;Y];X])->Self{
		Self(value)
	}
	fn dot<const Z:usize>(self,other:Array2d<Y,Z>)->Array2d<X,Z>{
		let mut tr=[[0f32;Y];Z];
		for y in 0..Y{
			for z in 0..Z{
				tr[z][y]=other.0[y][z];
			}
		}
		Array2d(self.0.map(|axis|
			tr.map(|trax|
				Array(axis).dot(Array(trax))
			)
		))
	}
}

#[derive(Clone,Copy)]
struct Matrix3<T>{
	x_axis:T,
	y_axis:T,
	z_axis:T,
}

impl std::ops::Deref for Array2d<3,3>{
	type Target=Matrix3<Vector3>;
	fn deref(&self)->&Self::Target{
		unsafe{std::mem::transmute(self)}
	}
}
impl std::ops::DerefMut for Array2d<3,3>{
	fn deref_mut(&mut self)->&mut Self::Target{
		unsafe{std::mem::transmute(self)}
	}
}

fn main(){
	let mut v=Array::new([1.0,2.0,3.0]);
	let dotty=v.dot(v);
	v.x*=5.0;//wow
	println!("dot={dotty}");
	println!("v.x={}",v.x);//wow!

	let mut m=Array2d::new([[1.0,2.0,3.0],[4.0,5.0,6.0],[7.0,8.0,9.0]]);
	let mdotty=m.dot(m);
	m.x_axis=*Array::new([-1.0,-2.0,-3.0]);
	m.x_axis.x=0.5;
	println!("mat mul xx={}",mdotty.x_axis.x);
	println!("m.x_axis.x={}",m.x_axis.x);
	println!("m.x_axis.y={}",m.x_axis.y);
}
