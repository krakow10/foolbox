use std::collections::HashSet;
use rbx_dom_weak::{types::Variant,ustr};

fn main(){
	let db=rbx_reflection_database::get();
	let base_script=&db.classes["LuaSourceContainer"];
	let file=std::fs::read(std::env::args().skip(1).next().unwrap()).unwrap();
	let dom=rbx_binary::from_reader(file.as_slice()).unwrap();

	let mut unique_scripts=HashSet::new();
	for instance in dom.descendants(){
		let class=db.classes.get(instance.class.as_str()).unwrap();
		if db.has_superclass(class,base_script){
			if let Some(Variant::String(source))=instance.properties.get(&ustr("Source")){
				unique_scripts.insert(source.as_str());
			}
		}
	}
	println!("unique scripts: {}",unique_scripts.len());
}
