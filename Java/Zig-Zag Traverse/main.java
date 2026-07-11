import java.util.ArrayDeque;
import java.util.Deque;

class main {
    private static class Node {
        final int value;
        Node left;
        Node right;

        Node(int value) { this.value = value; }
    }

    private static void zigZag(Node root) {
        Deque<Node> current = new ArrayDeque<Node>();
        Deque<Node> next = new ArrayDeque<Node>();
        boolean leftToRight = true;
        current.push(root);
        while (!current.isEmpty()) {
            Node node = current.pop();
            System.out.print(node.value + " ");
            if (leftToRight) {
                if (node.right != null) next.push(node.right);
                if (node.left != null) next.push(node.left);
            } else {
                if (node.left != null) next.push(node.left);
                if (node.right != null) next.push(node.right);
            }
            if (current.isEmpty()) {
                Deque<Node> temporary = current;
                current = next;
                next = temporary;
                leftToRight = !leftToRight;
            }
        }
        System.out.println();
    }

    public static void main(String[] args) {
        Node root = new Node(1);
        root.left = new Node(2); root.right = new Node(3);
        root.left.left = new Node(4); root.left.right = new Node(5);
        root.right.left = new Node(6); root.right.right = new Node(7);
        zigZag(root);
    }
}
