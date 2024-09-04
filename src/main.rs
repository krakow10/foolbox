// Array-backed vector with named field access

#[derive(Clone,Copy)]
struct Array<const N:usize>([f32;N]);
impl<const N:usize> Array<N>{
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

#[derive(Clone,Copy)]
union MagicVector3 {
	array:Array<3>,
	named:Vector3,
}

impl MagicVector3{
	fn dot(self,other:MagicVector3)->f32{
		unsafe{
			self.array.dot(other.array)
		}
	}
}

impl std::ops::Deref for MagicVector3{
	type Target=Vector3;
	fn deref(&self)->&Self::Target{
		unsafe{
			&self.named
		}
	}
}
impl std::ops::DerefMut for MagicVector3{
	fn deref_mut(&mut self)->&mut Self::Target{
		unsafe{
			&mut self.named
		}
	}
}

impl From<[f32;3]> for MagicVector3{
	fn from(value:[f32;3])->Self{
		Self{
			array:Array(value)
		}
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
struct Matrix3{
	x_axis:MagicVector3,
	y_axis:MagicVector3,
	z_axis:MagicVector3,
}

#[derive(Clone,Copy)]
union MagicMatrix3 {
	array:Array2d<3,3>,
	named:Matrix3,
}

impl MagicMatrix3{
	fn dot(self,other:MagicMatrix3)->MagicMatrix3{
		MagicMatrix3{
			array:unsafe{
				self.array.dot(other.array)
			}
		}
	}
}

impl std::ops::Deref for MagicMatrix3{
	type Target=Matrix3;
	fn deref(&self)->&Self::Target{
		unsafe{
			&self.named
		}
	}
}
impl std::ops::DerefMut for MagicMatrix3{
	fn deref_mut(&mut self)->&mut Self::Target{
		unsafe{
			&mut self.named
		}
	}
}

impl From<[[f32;3];3]> for MagicMatrix3{
	fn from(value:[[f32;3];3])->Self{
		Self{
			array:Array2d(value)
		}
	}
}

fn main(){
	let mut v=MagicVector3::from([1.0,2.0,3.0]);
	let dotty=v.dot(v);
	v.x*=5.0;//wow
	println!("dot={dotty}");
	println!("v.x={}",v.x);//wow!

	let mut m=MagicMatrix3::from([[1.0,2.0,3.0],[4.0,5.0,6.0],[7.0,8.0,9.0]]);
	let mdotty=m.dot(m);
	m.x_axis=MagicVector3::from([-1.0,-2.0,-3.0]);
	println!("mat mul xx={}",mdotty.x_axis.x);
	println!("m.x_axis.x={}",m.x_axis.x);
}
