use std::cell::Cell;
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

fn merge(p1: &Vec<i32>, p2: &Vec<i32>, nend: usize, end: usize) -> Vec<i32> {
    let mut i = 0;
    let mut j = 0;
    let mut vn: Vec<i32> = Vec::new();
    while i < nend && j < end - nend {
        if p1[i] > p2[j] {
            vn.push(p2[j]);
            j += 1;
        } else {
            vn.push(p1[i]);
            i += 1;
        }
    }
    while i < nend //if not finished
    {
        vn.push(p1[i]);
        i += 1;
    }
    while j < end - nend //because second array might be bigger
    {
        vn.push(p2[j]);
        j += 1;
    }
    vn
}

fn merge_sort(v: &Vec<i32>) -> Vec<i32> {
    if v.len() > 1 {
        let nend = v.len() / 2; // integer division already rounds down
        let end = v.len();
        let mut p1: Vec<i32> = Vec::new();
        let mut p2: Vec<i32> = Vec::new();
        for i in 0..nend {
            p1.push(v[i]);
        }
        for i in nend..end {
            p2.push(v[i]);
        }

        p1 = merge_sort(&p1);
        p2 = merge_sort(&p2);

        merge(&p1, &p2, nend, end)
    } else {
        v.clone()
    }
}

fn main() {
    srand(time_now());
    let mut v: Vec<i32> = Vec::new();
    print!("Array:  ");
    v.push(23);
    for _ in 0..25 {
        let d = rand() % 100 + 1;
        v.push(d);
        print!("{} ", d);
    }
    v = merge_sort(&v);
    print!("\n\nSorted Array:  ");
    for it in &v {
        print!("{} ", it);
    }
    print!("\n\n");
}
