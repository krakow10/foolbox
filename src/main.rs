use std::collections::HashMap;

fn main(){
	struct Battle{
		timeout:f32,
		player:&'static str,
	}

	let mut battles=HashMap::new();
	battles.insert("BlubBlub",
		Battle{
			timeout:5.0,
			player:"Quaternions",
		}
	);

	let time_now=6.0;

	for (fish,battle) in battles.iter_mut(){
		if battle.timeout<time_now{
			battles.remove(fish);
		}
	}
}
