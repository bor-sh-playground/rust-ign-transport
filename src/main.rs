#[link(name = "gz_wrapper")]
extern {
    pub fn run();
}

fn main() {
    unsafe {
        run();
    }
}