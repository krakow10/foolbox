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

// Mat<3,Vec<2>>.dot(M)
// M.x_axis

fn main(){
	let mut v=MagicVector3{array:Array([1.0,2.0,3.0])};
	let dotty=v.dot(v);
	v.x=5.0;
	println!("dot={dotty}");
	println!("v.x={}",v.x);//wow!
}
