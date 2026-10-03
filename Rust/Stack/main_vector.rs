//const MAX_SIZE: usize = 10;

struct Stack {
    data: Vec<i32>,
    //data: [i32; MAX_SIZE],
    //ptr: usize,
}

/*fn init(s: &mut Stack) {
    s.ptr = 0;
}*/

fn is_empty(s: &Stack) -> bool {
    if s.data.len() == 0 {
        return true;
    }
    false
}

fn is_full(s: &Stack) -> bool {
    // a Vec can hold at most isize::MAX bytes, this is Rust's vector::max_size()
    let max_size = isize::MAX as usize / std::mem::size_of::<i32>();
    if s.data.len() == max_size {
        return true;
    }
    false
}

fn push(s: &mut Stack, data: i32) {
    if !is_full(s) {
        s.data.push(data);
    } else {
        println!("Stack is full!");
    }
}

fn pop(s: &mut Stack) -> i32 {
    if !is_empty(s) {
        let temp = s.data[s.data.len() - 1];
        s.data.pop();
        temp
    } else {
        println!("Stack is empty!");
        -1 // the C++ version returns nothing here (undefined behaviour); Rust needs a value
    }
}

fn main() {
    let mut s = Stack { data: Vec::new() };
    let mut i = 0;
    while i < 10 {
        println!("I am adding {} to stack", i);
        push(&mut s, i);
        i += 1;
    }
    while !is_empty(&s) {
        println!("I have popped out {} from stack", pop(&mut s));
    }
}
