trait Z{
    fn z(self);
}
// function x has generic type Y with trait bound Z
fn x<Y:Z>(y:Y){
    y.z()
}

fn main(){
    struct Fella;
    impl Z for Fella{
        fn z(self){
            println!("hello from Y using Z");
        }
    }
    // allowed by trait bounds
    x(Fella);

    struct Goober;
    // not allowed, Goober does not implement Z
    // x(Goober); // compile error
}
