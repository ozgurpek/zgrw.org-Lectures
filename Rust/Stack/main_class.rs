//A generic stack declaration with a class for any type of data

// No #[derive(Clone)]: copying is forbidden, like the private copy
// constructor and assignment operator in the C++ version.
struct Stack<T> {
    data: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Stack<T> {
        //do nothing for now
        Stack { data: Vec::new() }
    }

    fn is_empty(&self) -> bool {
        if self.data.len() == 0 {
            return true;
        }
        false
    }

    fn is_full(&self) -> bool {
        // a Vec can hold at most isize::MAX bytes, this is Rust's vector::max_size()
        let max_size = isize::MAX as usize / std::mem::size_of::<T>().max(1);
        if self.data.len() == max_size {
            return true;
        }
        false
    }

    fn push(&mut self, data: T) {
        if !self.is_full() {
            self.data.push(data);
        } else {
            println!("Stack is full!");
        }
    }

    // The C++ version returns nothing when the stack is empty (undefined behaviour).
    // Here None means the stack was empty.
    fn pop(&mut self) -> Option<T> {
        if !self.is_empty() {
            self.data.pop()
        } else {
            println!("Stack is empty!");
            None
        }
    }
}

// the destructor
impl<T> Drop for Stack<T> {
    fn drop(&mut self) {
        //do nothing for now
    }
}

fn main() {
    let mut s: Stack<i32> = Stack::new();
    let mut i = 0;
    while i < 10 {
        println!("I am adding {} to stack", i);
        s.push(i);
        i += 1;
    }
    while !s.is_empty() {
        println!("I have popped out {} from stack", s.pop().unwrap());
    }
}
