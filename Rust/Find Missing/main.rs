use std::cell::Cell;
use std::time::{SystemTime, UNIX_EPOCH};

const MAX: i32 = 250;

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

fn find_missing(vec: &Vec<i32>, max: i32) -> i32 //O(N) solution
{
    let mut remain = (max * (max + 1)) / 2;
    for i in 0..vec.len() //O(n) loop
    {
        remain -= vec[i];
    }
    remain
}

// Rust has no function overloading, so the second findMissing gets its own name
fn find_missing_sorted(vec: &mut Vec<i32>) -> i32 //O(nlogn) solution
{
    vec.sort(); //O(nlogn) sort
    let mut i = 0;
    while i < vec.len() //O(n) loop
    {
        if vec[i] != i as i32 + 1 {
            break;
        }
        i += 1;
    }
    i as i32 + 1 //because i starts from 0
}

fn find_missing_non_unique(vec: &mut Vec<i32>) -> i32 //O(nlogn) solution
{
    vec.sort(); //O(nlogn) sort
    let mut look_up = 1;
    for i in 0..vec.len() //O(n) loop
    {
        if vec[i] > look_up {
            break;
        } else if vec[i] == look_up {
            look_up += 1;
        }
    }
    look_up
}

fn main() {
    srand(time_now());
    let missing = rand() % MAX + 1;
    println!("Missing should be found as: {}", missing);
    let mut v: Vec<i32> = Vec::new();
    print!("Array is: ");
    for i in 1..=MAX {
        if i != missing {
            v.push(i);
            print!("{} ", i);
        }
    }
    print!("\n\n");
    println!("Missing value is (O(N)): {}", find_missing(&v, MAX));
    println!("Missing value is (O(nlogn)): {}", find_missing_sorted(&mut v));
    v.clear();
    print!("\n\n");
    print!("Non-unique array is: ");
    for i in 1..=MAX {
        let d = rand() % 3 + 1; // max three occurences
        let mut j = 0;
        while j < d && i != missing {
            v.push(i);
            print!("{} ", i);
            j += 1;
        }
    }
    print!("\n\n");
    println!("Missing non-unique value is (O(nlogn)): {}", find_missing_non_unique(&mut v));
}
