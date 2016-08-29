#[link(name = "ign_wrapper")]
extern {
    pub fn run();
    pub fn print_my_stuff();
    pub fn waitForShutdown();
}

fn main() {
    unsafe {
        run();
        waitForShutdown();
    }
}