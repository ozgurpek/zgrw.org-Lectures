const DEF_SIZE: usize = 20;

// T: Copy + Into<i64> plays the role of std::is_arithmetic<T> (integer types only)
struct HashMap<T> {
    size: usize,
    v: Vec<Vec<T>>, // each bucket is a chain (std::list<T> in C++)
}

impl<T: Copy + PartialEq + Into<i64>> HashMap<T> {
    #[allow(dead_code)] // not used by main, kept for parity with C++
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

    fn remove(&mut self, data: T) {
        let idx = self.hash(data);
        self.v[idx].retain(|x| *x != data); // removes every equal element, like list::remove
    }

    fn search(&self, data: T) -> bool {
        let idx = self.hash(data);
        self.v[idx].contains(&data) //check if it is in the list or not and return the result
    }

    fn hash(&self, data: T) -> usize {
        (data.into() % self.size as i64) as usize
    }
}

fn main() {
    let mut hm: HashMap<i32> = HashMap::with_size(10);
    hm.insert(10);
    hm.insert(20);
    hm.insert(30);
    hm.insert(40);
    hm.insert(50);
    hm.insert(60);
    hm.insert(70);
    hm.insert(80);
    hm.insert(90);
    hm.insert(1);
    hm.insert(2);
    hm.insert(3);
    hm.insert(4);
    hm.insert(5);
    hm.insert(6);
    hm.insert(7);
    hm.insert(8);
    hm.insert(9);
    hm.insert(11);
    hm.insert(22);
    hm.insert(33);
    hm.insert(44);
    hm.insert(55);
    hm.insert(66);
    hm.insert(77);
    hm.insert(88);
    hm.insert(99);

    // `as i32` prints true/false as 1/0, like cout does with bool
    println!("Search for 10: {}", hm.search(10) as i32);
    println!("Search for 19: {}", hm.search(19) as i32);
    println!("Search for 99: {}", hm.search(99) as i32);
    println!("Search for 9: {}", hm.search(9) as i32);
    println!("Search for 20: {}", hm.search(20) as i32);
    println!("Search for 101: {}", hm.search(101) as i32);
    hm.remove(9);
    println!("Search for 9: {}", hm.search(9) as i32);
}
