// A missing child (NULL in C++) is None; Box owns the child node.
struct Node {
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    data: i32,
}

impl Node {
    fn new(data: i32) -> Node {
        Node { left: None, right: None, data }
    }
}

fn zigzag(root: &Node) {
    // two Vecs used as stacks (push / pop at the end)
    let mut s1: Vec<&Node> = Vec::new();
    let mut s2: Vec<&Node> = Vec::new();
    s1.push(root);
    while !s1.is_empty() || !s2.is_empty() {
        while let Some(temp) = s1.pop() {
            print!("{} ", temp.data);
            if let Some(right) = &temp.right {
                s2.push(right);
            }
            if let Some(left) = &temp.left {
                s2.push(left);
            }
        }

        while let Some(temp) = s2.pop() {
            print!("{} ", temp.data);
            if let Some(left) = &temp.left {
                s1.push(left);
            }
            if let Some(right) = &temp.right {
                s1.push(right);
            }
        }
    }
}

fn main() {
    let mut root = Node::new(1);
    root.left = Some(Box::new(Node::new(2)));
    root.right = Some(Box::new(Node::new(3)));

    let left = root.left.as_mut().unwrap();
    left.left = Some(Box::new(Node::new(4)));
    left.right = Some(Box::new(Node::new(5)));

    let right = root.right.as_mut().unwrap();
    right.left = Some(Box::new(Node::new(6)));
    right.right = Some(Box::new(Node::new(7)));
    zigzag(&root);
}
