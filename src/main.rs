fn main(){
	let mut map=indexmap::IndexSet::new();
	for i in 0..128{
		let (index,_)=map.insert_full(i);
		assert_eq!(index,i);
	}
	for (index,i) in map.into_iter().enumerate(){
		assert_eq!(index,i);
	}
}
