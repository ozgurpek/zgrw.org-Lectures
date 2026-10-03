// C-style singly linked list: free functions that take the head and return the new head.
// A raw `list *` becomes `Link` (Option<Box<List>>): None is NULL, Box owns the node,
// and dropping a Box is the `delete`.

struct List {
    next: Link,
    data: i32,
}

type Link = Option<Box<List>>;

fn init() -> Link {
    None
}

fn insert(head: Link, d: i32) -> Link {
    match head {
        None => Some(Box::new(List { next: None, data: d })),
        Some(mut node) => {
            node.next = insert(node.next.take(), d);
            Some(node)
        }
    }
}

fn search_and_remove(mut head: Link, d: i32) -> Link {
    // ptr points at the link (head or some node's `next`) that may hold the value
    let mut ptr = &mut head;
    while ptr.as_ref().map_or(false, |node| node.data != d) {
        ptr = &mut ptr.as_mut().unwrap().next;
    }
    if let Some(node) = ptr.take() {
        *ptr = node.next; // unlink it; the removed node is freed here
    }
    head
}

fn remove_head(head: Link, d: &mut i32) -> Link {
    match head {
        Some(node) => {
            *d = node.data;
            node.next
        }
        None => None,
    }
}

#[allow(dead_code)] // not used by main, kept for parity with C++
fn remove_last(mut head: Link, d: &mut i32) -> Link {
    if head.is_some() {
        // walk until ptr is the link that holds the last node
        let mut ptr = &mut head;
        while ptr.as_ref().unwrap().next.is_some() {
            ptr = &mut ptr.as_mut().unwrap().next;
        }
        let last = ptr.take().unwrap(); // previous node's next (or head) becomes None
        *d = last.data;
    }
    head
}

fn main() {
    let mut head: Link = init();
    let mut i = 0;
    while i < 10 {
        println!("I am adding {} to list", i);
        head = insert(head, i);
        i += 1;
    }
    head = search_and_remove(head, 0);
    head = search_and_remove(head, 5);
    head = search_and_remove(head, 9);
    while head.is_some() {
        let mut d = 0;
        head = remove_head(head, &mut d);
        println!("I have removed {} from list", d);
    }
}
