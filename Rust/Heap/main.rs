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

struct Heap<T> //min heap implementation
{
    v: Vec<T>,
}

impl<T: PartialOrd + Copy> Heap<T> {
    fn new() -> Heap<T> {
        Heap { v: Vec::new() }
    }

    fn insert(&mut self, data: T) {
        let idx = self.v.len();
        self.v.push(data);
        self.heapify(idx);
    }

    #[allow(dead_code)] // not used by main, kept for parity with C++
    fn get_min(&self) -> T {
        self.v[0]
    }

    fn remove_min(&mut self) -> T {
        let idx = self.v.len() - 1;
        self.v.swap(idx, 0);
        let val = self.v[idx];
        self.v.pop();
        self.heapify_del(0);
        val
    }

    fn is_empty(&self) -> bool {
        self.v.is_empty()
    }

    fn heapify(&mut self, idx: usize) {
        if idx != 0 //combo breaker
        {
            let parrent = if idx % 2 == 0 { idx / 2 } else { (idx - 1) / 2 };
            if self.v[idx] < self.v[parrent] {
                self.v.swap(idx, parrent);
                self.heapify(parrent);
            }
        }
    }

    fn heapify_del(&mut self, idx: usize) {
        let n = self.v.len() as i64;
        // (n - 1) / 2 is integer division, so the C++ ceil() does not change it
        if n > 1 && idx as i64 <= (n - 1) / 2 - 1 {
            let ch1 = if idx == 0 { 1 } else { idx * 2 };
            let ch2 = ch1 + 1;
            let m_idx = if self.v[ch1] < self.v[ch2] { ch1 } else { ch2 };

            if self.v[idx] > self.v[m_idx] {
                self.v.swap(idx, m_idx);
                self.heapify_del(m_idx);
            }
        }
    }
}

fn main() {
    srand(time_now());
    let mut h: Heap<i32> = Heap::new();
    println!("Random array;");
    for _ in 0..11 {
        let d = rand() % 20 + 1;
        h.insert(d);
        print!("{} ", d);
    }
    print!("\n\n");
    println!("Heap sorted array;");
    while !h.is_empty() //Here an example of heap sort :)
    {
        println!("{}", h.remove_min());
    }
}
