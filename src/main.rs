#[link(name = "gz_wrapper")]
extern {
    pub fn run();
    pub fn pub_pos();
}

fn main() {
    unsafe {
        pub_pos();
//        run();
    }
}