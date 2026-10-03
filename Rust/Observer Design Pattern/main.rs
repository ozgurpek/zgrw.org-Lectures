// Build with: rustc --edition 2021 main.rs   (classes.rs is picked up by `mod classes;`)
mod classes;

use classes::{DivObserver, ModObserver, Subject};

fn main() {
    let s = Subject::new();
    let _d1 = DivObserver::new(&s, 3);
    let _m1 = ModObserver::new(&s, 7);
    let _d2 = DivObserver::new(&s, 4);
    let _m2 = ModObserver::new(&s, 5);
    s.set_val(10);
}
