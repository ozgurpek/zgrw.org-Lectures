const SIZE: i32 = 10;

// Generic circular queue. beg and end are i32 (not usize) because -1 means "not set".
// T: Copy + Default lets us fill the fixed size array with a start value.
struct Queue<T> {
    beg: i32,
    end: i32,
    data: [T; SIZE as usize],
}

impl<T: Copy + Default> Queue<T> {
    fn new() -> Queue<T> {
        Queue {
            beg: -1,
            end: -1,
            data: [T::default(); SIZE as usize],
        }
    }

    fn is_full(&self) -> bool {
        if (self.end == SIZE - 1 && self.beg == 0) || (self.end + 1 == self.beg && self.end != -1) {
            return true;
        }
        false
    }

    fn is_empty(&self) -> bool {
        self.beg == -1
    }

    fn insert(&mut self, d: T) {
        if !self.is_full() {
            if self.is_empty() {
                self.beg = self.end;
            }
            self.end += 1;
            if (self.end == SIZE && self.beg == 0) || self.end == self.beg {
                self.end -= 1;
            } else if self.end == SIZE && self.beg != 0 {
                self.end = 0;
            }
            self.data[self.end as usize] = d;
        }
    }

    // The C++ version returns -1 for an empty queue, which only works when T is a number.
    // For any T we return Option<T>: None means the queue was empty.
    fn remove(&mut self) -> Option<T> {
        if !self.is_empty() {
            let d = self.data[self.beg as usize];
            self.beg += 1;
            if self.beg == self.end + 1 {
                self.beg = -1;
            } else if self.beg == SIZE {
                self.beg = 0;
            }
            return Some(d);
        }
        None
    }
}

fn main() {
    let mut q: Queue<i32> = Queue::new();
    let mut i = 0;
    while !q.is_full() {
        println!("I am adding {} to queue", i);
        q.insert(i);
        i += 1;
    }
    while !q.is_empty() {
        println!("I have removed {} from queue", q.remove().unwrap());
    }
}
