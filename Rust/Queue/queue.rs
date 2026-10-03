const SIZE: usize = 10;

struct Queue {
    end: usize,
    data: [i32; SIZE],
}

fn init(q: &mut Queue) {
    q.end = 0;
}

fn is_full(q: &Queue) -> bool {
    q.end == SIZE
}

fn is_empty(q: &Queue) -> bool {
    q.end == 0
}

fn insert(q: &mut Queue, d: i32) {
    if !is_full(q) {
        q.data[q.end] = d;
        q.end += 1;
    }
}

fn shift_queue(q: &mut Queue) {
    for i in 0..q.end - 1 {
        q.data[i] = q.data[i + 1];
    }
    q.end -= 1;
}

fn remove(q: &mut Queue) -> i32 {
    if !is_empty(q) {
        let d = q.data[0];
        shift_queue(q);
        return d;
    }
    -1
}

fn main() {
    // Rust does not allow uninitialized memory, so the fields get a start value here
    let mut q = Queue { end: 0, data: [0; SIZE] };
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
