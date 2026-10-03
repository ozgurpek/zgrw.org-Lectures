// Generic version of list_class.rs: the node can hold any data type T.
// The C++ methods are called on a `list<T> *` and return the new head; here they
// take the node by value (`self: Box<Self>`) and hand back the new head, so the
// list can never be used after one of its nodes was deleted.

struct List<T> {
    next: Option<Box<List<T>>>,
    data: T,
}

// friend function: creates the first node if head is empty
fn init<T: PartialEq + Copy>(head: Option<Box<List<T>>>, d: T) -> Option<Box<List<T>>> {
    match head {
        None => Some(Box::new(List::new(d))),
        Some(node) => Some(node),
    }
}

impl<T: PartialEq + Copy> List<T> {
    // private constructor
    fn new(d: T) -> List<T> {
        List { next: None, data: d }
    }

    fn insert(mut self: Box<Self>, d: T) -> Box<List<T>> {
        self.next = match self.next.take() {
            None => init(None, d),
            Some(next) => Some(next.insert(d)),
        };
        self
    }

    fn search_and_remove(mut self: Box<Self>, d: T) -> Option<Box<List<T>>> {
        if self.data == d {
            return self.next.take(); // the head itself is deleted
        }
        // ptr points at the `next` link that may hold the value
        let mut ptr = &mut self.next;
        while ptr.as_ref().map_or(false, |node| node.data != d) {
            ptr = &mut ptr.as_mut().unwrap().next;
        }
        if let Some(node) = ptr.take() {
            *ptr = node.next;
        }
        Some(self)
    }

    fn remove_head(self: Box<Self>, d: &mut T) -> Option<Box<List<T>>> {
        *d = self.data;
        self.next // `self` (the old head) is freed when this returns
    }

    #[allow(dead_code)] // not used by main, kept for parity with C++
    fn remove_last(mut self: Box<Self>, d: &mut T) -> Option<Box<List<T>>> {
        if self.next.is_none() {
            *d = self.data;
            return None; // the only node is deleted
        }
        let mut ptr = &mut self.next;
        while ptr.as_ref().unwrap().next.is_some() {
            ptr = &mut ptr.as_mut().unwrap().next;
        }
        let last = ptr.take().unwrap();
        *d = last.data;
        Some(self)
    }
}

fn main() {
    let mut head: Option<Box<List<f64>>> = None;
    head = init(head, 0.0);
    let mut i: f64 = 1.0;
    while i < 10.0 {
        println!("I am adding {} to list", i);
        head = head.map(|h| h.insert(i)); // only called when head is not NULL
        i += 1.0;
    }
    head = head.and_then(|h| h.search_and_remove(0.0));
    head = head.and_then(|h| h.search_and_remove(5.0));
    head = head.and_then(|h| h.search_and_remove(9.0));
    while let Some(h) = head {
        let mut d = 0.0;
        head = h.remove_head(&mut d);
        println!("I have removed {} from list", d);
    }
}
