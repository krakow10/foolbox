// Array-backed vector with named field access

#[derive(Clone,Copy)]
struct Array<const N:usize,T>([T;N]);
impl<const N:usize,T> Array<N,T>{
	const fn new(value:[T;N])->Self{
		Self(value)
	}
}
impl<const N:usize,T> Array<N,T>
	where
		T:std::ops::Mul+std::iter::Sum<<T as std::ops::Mul>::Output>,
{
	fn dot(self,other:Self)->T{
		self.0.into_iter().zip(other.0).map(|(a,b)|a*b).sum()
	}
}

#[derive(Clone,Copy)]
#[repr(C)]
struct Vector3<T>{
	x:T,
	y:T,
	z:T,
}


impl<T> std::ops::Deref for Array<3,T>{
	type Target=Vector3<T>;
	fn deref(&self)->&Self::Target{
		unsafe{std::mem::transmute(self)}
	}
}
impl<T> std::ops::DerefMut for Array<3,T>{
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
struct Array2d<const X:usize,const Y:usize,T>([[T;Y];X]);
impl<const X:usize,const Y:usize,T> Array2d<X,Y,T>{
	const fn new(value:[[T;Y];X])->Self{
		Self(value)
	}
}
impl<const X:usize,const Y:usize> Array2d<X,Y,f32>{
	fn dot<const Z:usize>(self,other:Array2d<Y,Z,f32>)->Array2d<X,Z,f32>{
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
#[repr(C)]
struct Matrix3<T>{
	x_axis:T,
	y_axis:T,
	z_axis:T,
}

impl<T> std::ops::Deref for Array2d<3,3,T>{
	type Target=Matrix3<Vector3<T>>;
	fn deref(&self)->&Self::Target{
		unsafe{std::mem::transmute(self)}
	}
}
impl<T> std::ops::DerefMut for Array2d<3,3,T>{
	fn deref_mut(&mut self)->&mut Self::Target{
		unsafe{std::mem::transmute(self)}
	}
}

fn main(){
	let mut v=Array::new([1.0,2.0,3.0]);

	// const-generic fixed-size vector dot
	let d=v.dot(v);
	println!("dot={d}");

	v.x*=5.0;//wow
	println!("v.x={}",v.x);//wow!

	let mut m=Array2d::new([[1.0,2.0,3.0],[4.0,5.0,6.0],[7.0,8.0,9.0]]);

	let md=m.dot(m);
	println!("mat mul xx={}",md.x_axis.x);

	m.x_axis=*Array::new([-1.0,-2.0,-3.0]);
	m.x_axis.x=0.5;
	println!("m.x_axis.x={}",m.x_axis.x);
	println!("m.x_axis.y={}",m.x_axis.y);
}
