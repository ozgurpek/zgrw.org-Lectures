const MAX_SIZE: usize = 10;

struct Stack {
    data: [i32; MAX_SIZE],
    ptr: usize,
}

fn init(s: &mut Stack) {
    s.ptr = 0;
}

fn is_empty(s: &Stack) -> bool {
    if s.ptr == 0 {
        return true;
    }
    false
}

fn is_full(s: &Stack) -> bool {
    if s.ptr == MAX_SIZE {
        return true;
    }
    false
}

fn push(s: &mut Stack, data: i32) {
    if !is_full(s) {
        s.data[s.ptr] = data;
        s.ptr += 1;
    } else {
        println!("Stack is full!");
    }
}

fn pop(s: &mut Stack) -> i32 {
    if !is_empty(s) {
        s.ptr -= 1;
        s.data[s.ptr]
    } else {
        println!("Stack is empty!");
        -1 // the C++ version returns nothing here (undefined behaviour); Rust needs a value
    }
}

fn main() {
    // Rust does not allow uninitialized memory, so the fields get a start value here
    let mut s = Stack { data: [0; MAX_SIZE], ptr: 0 };
    init(&mut s);
    let mut i = 0;
    while !is_full(&s) {
        println!("I am adding {} to stack", i);
        push(&mut s, i);
        i += 1;
    }
    while !is_empty(&s) {
        println!("I have popped out {} from stack", pop(&mut s));
    }
}
