struct Unit{
	referent:u128,
}

struct Instance{
	name:String,
}
impl Instance{
	fn add_fields<'lua,T,F:mlua::UserDataFields<'lua,T>+?Sized>(fields:&mut F){}
}
impl mlua::UserData for Instance{
	fn add_fields<'lua,F:mlua::UserDataFields<'lua,Self>>(fields:&mut F){
		Instance::add_fields(fields);
	}
}

struct DataModel{
}
impl DataModel{
	fn add_fields<'lua,T,F:mlua::UserDataFields<'lua,T>+?Sized>(fields:&mut F){}
}

impl mlua::UserData for DataModel{
	fn add_fields<'lua,F:mlua::UserDataFields<'lua,Self>>(fields:&mut F){
		Instance::add_fields(fields);
		DataModel::add_fields(fields);
	}
}

fn main() {
}
