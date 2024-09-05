struct Five(u8);
impl Default for Five{
	fn default() -> Self {
		Five(5)
	}
}
fn main() {
	//arrays only implement default up to length 32 !?
	let arr:[Five;32]=Default::default();
	let arr:[Five;33]=Default::default();
}
