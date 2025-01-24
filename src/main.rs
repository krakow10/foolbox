use std::collections::HashMap;

fn main(){
	let data=include_bytes!("../meshdata/385416572.meshdata");
	let columns=84;
	let modulus=31;
	let mut c=0;
	let mut tally:[HashMap<u8,u32>;31]=Default::default();
	for (y,chunk) in data.chunks_exact(columns).enumerate(){
		for (x,&b) in chunk.iter().enumerate(){
			c+=1;
			if 34<=x&&x<=51&&1<y&&y<64{
				let i=c%modulus;
				*tally[i].entry(b).or_insert(0)+=1;
			}
		}
	}
	for (i,h) in tally.into_iter().enumerate(){
		let mut k_max_v=0;
		let mut max_v=0;
		for (k,v) in h{
			if max_v<v{
				k_max_v=k;
				max_v=v;
			}
		}
		println!("noise[{i}]={k_max_v}");
	}
	let noise=[122,86,46,110,88,49,32,48,4,52,105,12,119,12,1,94,0,26,96,55,105,29,82,43,7,79,36,89,101,83,4];
}
