fn main(){
	let bytes=include_bytes!("/run/media/quat/Files/Documents/map-files/verify-scripts/maps/surf_all/surf_clique2.rbxm");
	rbx_binary::from_reader(bytes.as_slice()).unwrap();
}
