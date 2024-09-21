fn a_function<'lua,T,F:mlua::UserDataFields<'lua,T>+?Sized>(fields:&mut F){}

pub trait Composition<'lua,F>:Sized
	where
		F:mlua::UserDataFields<'lua,Self>+'lua,
{
	const COMPOSITION:&'lua [fn(&mut F)];
}

struct Unit{
	referent:u128,
}

struct Instance{
	name:String,
}
impl Instance{}

impl<'a,F> Composition<'a,F> for Instance
where
	F:mlua::UserDataFields<'a,Self>+'a{
	const COMPOSITION:&'a [fn(&mut F)]=&[a_function];
}

impl mlua::UserData for T
{
	fn add_fields<'lua,F>(fields:&mut F)
	where
			F:mlua::UserDataFields<'lua,Self>+'lua,
			T:Composition<'lua,F>,
	{
		T::COMPOSITION.iter().for_each(|f|f(fields))
	}
}

fn main() {
	let mut a=DataModel::new();
	a=Lighting::new();
}
