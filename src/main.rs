//#[link(name = "doubler")]
extern {
    fn doubler(x: i32) -> i32;
}
/*
fn doubler(x: usize) -> usize {
    x*2
}
*/
fn main() {
    unsafe {
	println!("rust: {}", doubler(1));
    }
}
