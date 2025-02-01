struct Node{
	left:Option<Box<Node>>,
	right:Option<Box<Node>>,
}
impl Node{
	fn invert(&mut self){
		core::mem::swap(&mut self.left,&mut self.right);
		if let Some(left)=&mut self.left{
			left.invert();
		}
		if let Some(right)=&mut self.right{
			right.invert();
		}
	}
	fn display(&self,f:&mut std::fmt::Formatter<'_>,depth:usize)->std::fmt::Result{
		write!(f,"{{\n")?;
		let new_depth=depth+1;
		if let Some(left)=&self.left{
			write!(f,"{}","\t".repeat(new_depth))?;
			write!(f,"left:")?;
			left.display(f,new_depth)?;
		}
		if let Some(right)=&self.right{
			write!(f,"{}","\t".repeat(new_depth))?;
			write!(f,"right:")?;
			right.display(f,new_depth)?;
		}
		write!(f,"{}","\t".repeat(depth))?;
		write!(f,"}}\n")?;
		Ok(())
	}
}
impl std::fmt::Display for Node{
	fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{
		self.display(f,0)
	}
}

fn main(){
	let d=Node{left:None,right:None};
	let b=Node{left:None,right:None};
	let c_d_=Node{left:Some(d.into()),right:None};
	let mut a_bc=Node{left:Some(b.into()),right:Some(c_d_.into())};
	println!("{}",a_bc);
	a_bc.invert();
	println!("{}",a_bc);
}
