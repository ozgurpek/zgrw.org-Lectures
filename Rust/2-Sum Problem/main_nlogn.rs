use std::cell::Cell;
use std::io::{self, Write};
use std::time::{SystemTime, UNIX_EPOCH};

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

// Rust has no function overloading, so quickSort(v, beg, end) gets its own name
fn quick_sort_range(v: &mut Vec<i32>, beg: usize, end: usize) {
    if end > beg + 1 {
        let mut pv = (rand() as usize % (end - beg)) + beg;
        for i in beg..end {
            if v[i] < v[pv] && i > pv {
                if i == pv + 1 {
                    v.swap(pv, i);
                    pv = i;
                } else {
                    v.swap(pv, pv + 1);
                    v.swap(pv, i);
                    pv += 1;
                }
            } else if v[i] > v[pv] && i < pv {
                v.swap(pv, i);
                pv = i;
            }
        }
        quick_sort_range(v, beg, pv);
        quick_sort_range(v, pv, end);
    }
}

fn quick_sort(v: &mut Vec<i32>) {
    let beg = 0;
    let end = v.len();
    quick_sort_range(v, beg, end);
}

fn main() {
    srand(time_now());
    let mut v: Vec<i32> = Vec::new();
    print!("Array:  ");
    for _ in 0..60 {
        let mut d = rand() % 16 + 1;
        if rand() % 2 == 1 {
            d *= -1;
        }
        v.push(d);
        print!("{} ", d);
    }
    quick_sort(&mut v);

    print!("\n\nEnter a number between 1 - 15 as sum: ");
    io::stdout().flush().unwrap();
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let target: i32 = line.trim().parse().unwrap_or(0); // like cin, a bad number becomes 0

    // indexes play the role of the C++ iterators
    let len = v.len() as i32;
    let mut itu: i32 = len - 1;
    let mut itl: i32 = 0;
    while itl < len && itu >= 0 && itl < itu {
        let temp = v[itl as usize] + v[itu as usize];
        if temp < target {
            itl += 1;
        } else if temp > target {
            itu -= 1;
        } else {
            println!("{} - {}", v[itl as usize], v[itu as usize]);
            itl += 1;
            itu -= 1;
        }
    }
}
