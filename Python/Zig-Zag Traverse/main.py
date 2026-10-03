from __future__ import annotations


class Node:
    def __init__(self, data: int = 0) -> None:
        self.left: Node | None = None
        self.right: Node | None = None
        self.data = data


def zigzag(root: Node) -> None:
    s1: list[Node] = []  # Python lists work as stacks: append() = push, pop() = top + pop
    s2: list[Node] = []
    s1.append(root)
    while s1 or s2:
        while s1:
            temp = s1.pop()
            print(temp.data, end=" ")
            if temp.right is not None:
                s2.append(temp.right)
            if temp.left is not None:
                s2.append(temp.left)

        while s2:
            temp = s2.pop()
            print(temp.data, end=" ")
            if temp.left is not None:
                s1.append(temp.left)
            if temp.right is not None:
                s1.append(temp.right)


def main() -> None:
    root = Node()
    root.data = 1
    root.left = Node()
    root.left.data = 2
    root.right = Node()
    root.right.data = 3
    root.left.left = Node()
    root.left.left.data = 4
    root.left.right = Node()
    root.left.right.data = 5

    root.right.left = Node()
    root.right.left.data = 6
    root.right.right = Node()
    root.right.right.data = 7
    zigzag(root)


if __name__ == "__main__":
    main()
