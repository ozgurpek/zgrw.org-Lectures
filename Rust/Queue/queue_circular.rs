const SIZE: i32 = 10;

// beg and end are i32 (not usize) because -1 means "not set"
struct Queue {
    beg: i32,
    end: i32,
    data: [i32; SIZE as usize],
}

fn init(q: &mut Queue) {
    q.beg = -1;
    q.end = -1;
}

fn is_full(q: &Queue) -> bool {
    if (q.end == SIZE - 1 && q.beg == 0) || (q.end + 1 == q.beg && q.end != -1) {
        return true;
    }
    false
}

fn is_empty(q: &Queue) -> bool {
    q.beg == -1
}

fn insert(q: &mut Queue, d: i32) {
    if !is_full(q) {
        if is_empty(q) {
            q.beg = q.end;
        }
        q.end += 1;
        if (q.end == SIZE && q.beg == 0) || q.end == q.beg {
            q.end -= 1;
        } else if q.end == SIZE && q.beg != 0 {
            q.end = 0;
        }
        q.data[q.end as usize] = d;
    }
}

fn remove(q: &mut Queue) -> i32 {
    if !is_empty(q) {
        let d = q.data[q.beg as usize];
        q.beg += 1;
        if q.beg == q.end + 1 {
            q.beg = -1;
        } else if q.beg == SIZE {
            q.beg = 0;
        }
        return d;
    }
    -1
}

fn main() {
    // Rust does not allow uninitialized memory, so the fields get a start value here
    let mut q = Queue { beg: 0, end: 0, data: [0; SIZE as usize] };
    init(&mut q);
    let mut i = 0;
    while !is_full(&q) {
        println!("I am adding {} to queue", i);
        insert(&mut q, i);
        i += 1;
    }
    while !is_empty(&q) {
        println!("I have removed {} from queue", remove(&mut q));
    }
}
