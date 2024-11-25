use futures::StreamExt;

#[tokio::main]
async fn main(){
	let list=[];
	let yah=futures::stream::iter(list)
	.map(|num:u32|async move{num+1})
	.buffer_unordered(list.len())
	.filter_map(|num|async move{Some(num)})
	.collect::<Vec<u32>>();
	println!("running future...");
	let a=yah.await;
	println!("future completed {:?}",a);
}
