use hash_str::hstr;
use hash_str::{HashStr,UnhashedStr,HashStrHost,HashStrCache};

fn main(){
	// string internment cache
	let lifetime_host=HashStrHost::new();
	let mut cache=HashStrCache::new();

	// string with hash calculated at compile time
	let hstr_static:&HashStr=hstr!("bruh");
	// string with hash calculated at run time
	// anonymous means it does not belong to any HashStrCache
	let hstr_runtime:&HashStr=&HashStr::anonymous("bruh".to_owned());

	// intern string into deduplication cache
	// does not allocate unless "bruh" is a new string
	let hstr_interned:&HashStr=cache.intern_with(&lifetime_host,"bruh");

	let mut map=hash_str::HashStrMap::default();
	map.insert(hstr_static,1);

	assert_eq!(map.get(hstr_static),Some(&1));
	assert_eq!(map.get(hstr_runtime),Some(&1));
	assert_eq!(map.get(hstr_interned),Some(&1));
	// use Borrow<UnhashedStr> : &HashStr trait bound to index HashMap
	// without needing to allocate a temporary HashStr
	assert_eq!(map.get(UnhashedStr::from_ref("bruh")),Some(&1));

	// free cache memory of interned strings
	// does not affect static or anonymous HashStrs
	drop(cache);
	drop(lifetime_host);
}
