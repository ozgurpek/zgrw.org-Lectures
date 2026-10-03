use std::cell::Cell;
use std::time::{SystemTime, UNIX_EPOCH};

const SIZE: usize = 50;

// Rust's standard library has no rand()/srand(), so this is a tiny stand-in
// (the classic linear congruential generator from the C standard's example).
thread_local! {
    static RAND_STATE: Cell<u32> = Cell::new(1);
}

fn srand(seed: u32) {
    RAND_STATE.with(|s| s.set(seed));
}

fn rand() -> i32 {
    RAND_STATE.with(|s| {
        let next = s.get().wrapping_mul(1103515245).wrapping_add(12345);
        s.set(next);
        ((next / 65536) % 32768) as i32
    })
}

// Same role as time(0): seconds since 1970
fn time_now() -> u32 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as u32)
        .unwrap_or(0)
}

fn max_sum(v: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for &val in v {
        if val > 0 {
            sum += val;
        }
    }
    sum
}

fn max_cont_seq(v: &Vec<i32>) -> i32 //Kaden's algorithm
{
    let mut sum_ends_here = 0;
    let mut sum_so_far = 0;
    for &val in v {
        sum_ends_here = 0.max(sum_ends_here + val);
        sum_so_far = sum_ends_here.max(sum_so_far);
    }
    sum_so_far
}

fn main() {
    srand(time_now());
    let mut v: Vec<i32> = Vec::new();
    for _ in 0..SIZE {
        let mut d = rand() % 16;
        if rand() % 2 == 1 {
            d *= -1;
        }
        v.push(d);
        print!("{} ", d);
    }
    print!("\n\n");
    println!("Max Sum: {}", max_sum(&v));
    println!("Max Cont Sum: {}", max_cont_seq(&v));
}
