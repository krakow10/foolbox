fn main(){
	let t1=[1,3,5];
	let t2=[2,4,6];
	let t_final:Vec<u32>=t1
	.into_iter()
	.zip(t2)
	.flat_map(|(v0,v1)| [v0,v1])
	.collect();
	println!("{:?}",t_final);
}