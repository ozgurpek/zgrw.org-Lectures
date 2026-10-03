use std::cell::Cell;
use std::io::{self, Write};
use std::time::{SystemTime, UNIX_EPOCH};

const DEF_SIZE: usize = 20;

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

// T: Copy + Into<i64> plays the role of std::is_arithmetic<T> (integer types only)
struct HashMap<T> {
    size: usize,
    v: Vec<Vec<T>>, // each bucket is a chain (std::list<T> in C++)
}

impl<T: Copy + PartialEq + Into<i64>> HashMap<T> {
    //Default coonstructor
    fn new() -> HashMap<T> {
        HashMap::with_size(DEF_SIZE)
    }

    //Define size
    fn with_size(size: usize) -> HashMap<T> {
        let mut v = Vec::new();
        for _ in 0..size {
            v.push(Vec::new());
        }
        HashMap { size, v }
    }

    fn insert(&mut self, data: T) {
        let idx = self.hash(data);
        self.v[idx].push(data);
    }

    fn search(&mut self, data: T) -> bool {
        let idx = self.hash(data);
        let res = self.v[idx].contains(&data); //check if it is in the list or not and return the result
        self.remove(data);
        res
    }

    fn remove(&mut self, data: T) {
        let idx = self.hash(data);
        self.v[idx].retain(|x| *x != data); // removes every equal element, like list::remove
    }

    fn hash(&self, data: T) -> usize {
        (data.into().abs() % self.size as i64) as usize
    }
}

fn main() {
    srand(time_now());
    let mut v: Vec<i32> = Vec::new();
    for _ in 0..DEF_SIZE * 3 {
        let mut d = rand() % 16;
        if rand() % 2 == 1 {
            d *= -1;
        }
        v.push(d);
        print!("{} ", d);
    }
    print!("\n\nEnter a number between 1 - 15 as sum: ");
    io::stdout().flush().unwrap();
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let target: i32 = line.trim().parse().unwrap_or(0); // like cin, a bad number becomes 0
    let mut hm: HashMap<i32> = HashMap::new();
    for &it in &v {
        let rm = target - it;
        if hm.search(rm) {
            println!("{} - {}", it, rm);
        }
        hm.insert(it);
    }
}
