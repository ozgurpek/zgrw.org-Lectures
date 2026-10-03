use std::cell::Cell;
use std::time::{SystemTime, UNIX_EPOCH};

const SIZE: usize = 26;

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

// A child that is NULL in C++ is None here; Box owns the child node.
struct Node {
    val: char,
    child: Vec<Option<Box<Node>>>,
}

impl Node {
    fn new() -> Node {
        let mut child = Vec::new();
        for _ in 0..SIZE {
            child.push(None);
        }
        Node { val: '\0', child }
    }
}

fn insert_trie(head: &mut Node, txt: &str) -> bool {
    let mut curr_node = head;
    let mut inserted = false;
    for c in txt.chars() {
        let idx = (c as u8 - b'a') as usize;
        if curr_node.child[idx].is_none() {
            let mut tmp = Node::new();
            tmp.val = c;
            curr_node.child[idx] = Some(Box::new(tmp));
            inserted = true;
        }
        curr_node = curr_node.child[idx].as_mut().unwrap();
    }
    inserted
}

fn is_exist(head: &Node, txt: &str) -> bool {
    let mut curr_node = head;
    let mut exists = true;
    for c in txt.chars() {
        let idx = (c as u8 - b'a') as usize;
        match &curr_node.child[idx] {
            None => {
                exists = false;
                break;
            }
            Some(next) => curr_node = next,
        }
    }
    exists
}

fn guess_str(head: &Node, txt: &str) -> Vec<String> {
    srand(time_now());
    let mut curr_node = head;
    let mut g: Vec<String> = Vec::new();
    if is_exist(head, txt) {
        for c in txt.chars() {
            let idx = (c as u8 - b'a') as usize;
            curr_node = curr_node.child[idx].as_ref().unwrap();
        }

        //Find all non-null Idx
        let mut idxs: Vec<usize> = Vec::new();
        for i in 0..SIZE {
            if curr_node.child[i].is_some() {
                idxs.push(i);
            }
        }

        //Select 3 random idxs
        for _ in 0..3 {
            let mut s = String::from(txt);
            let mut idx = rand() as usize % idxs.len();
            idx = idxs[idx];
            let mut next: Option<&Node> = curr_node.child[idx].as_deref();
            s.push(next.unwrap().val);
            while let Some(mut n) = next {
                let mut found = false;
                for i in 0..SIZE {
                    // note: n moves down as soon as a child is found and the
                    // loop goes on with the child's children, just like the C++ code
                    if let Some(c) = &n.child[i] {
                        n = c;
                        found = true;
                        s.push(n.val);
                    }
                }
                next = if found { Some(n) } else { None };
            }
            if !g.contains(&s) {
                g.push(s);
            }
        }
    }
    g
}

fn main() {
    let mut head = Node::new();
    insert_trie(&mut head, "test");
    println!("{}", is_exist(&head, "test") as i32);
    insert_trie(&mut head, "tabak");
    insert_trie(&mut head, "turta");
    insert_trie(&mut head, "tup");
    println!("Second insert {}", insert_trie(&mut head, "tup") as i32);
    insert_trie(&mut head, "araba");
    insert_trie(&mut head, "bebek");
    insert_trie(&mut head, "ceket");
    insert_trie(&mut head, "deniz");
    insert_trie(&mut head, "etek");
    insert_trie(&mut head, "fasulye");
    insert_trie(&mut head, "gezi");
    insert_trie(&mut head, "harita");
    insert_trie(&mut head, "iplik");
    insert_trie(&mut head, "jakuzi");
    insert_trie(&mut head, "kalem");
    insert_trie(&mut head, "lale");
    insert_trie(&mut head, "muzik");
    insert_trie(&mut head, "nese");
    insert_trie(&mut head, "okul");
    insert_trie(&mut head, "parke");
    insert_trie(&mut head, "quadro");
    insert_trie(&mut head, "rize");
    insert_trie(&mut head, "sokak");
    insert_trie(&mut head, "tarak");
    insert_trie(&mut head, "tarti");
    insert_trie(&mut head, "taka");
    insert_trie(&mut head, "tasa");
    insert_trie(&mut head, "taki");
    insert_trie(&mut head, "tatil");
    insert_trie(&mut head, "ucak");
    insert_trie(&mut head, "veli");
    insert_trie(&mut head, "win");
    insert_trie(&mut head, "xavier");
    insert_trie(&mut head, "yeni");
    insert_trie(&mut head, "zeki");
    let g = guess_str(&head, "ta"); //word guesses starting with ta
    for s in &g {
        println!("{}", s);
    }
}
