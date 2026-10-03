"""Singly linked list with free functions (C-style), like list.cc.

Python has no pointers or `delete`: a node reference plays the role of a
pointer, None plays the role of NULL, and removed nodes are freed by the
garbage collector. C++ `int &d` out-parameters become extra return values.
"""

from __future__ import annotations


class Node:  # "struct list" in the C++ version
    def __init__(self, data: int):
        self.next: Node | None = None
        self.data = data


def init(head: Node | None) -> Node | None:
    head = None
    return head


def insert(head: Node | None, d: int) -> Node:
    if head is None:
        head = Node(d)
    else:
        head.next = insert(head.next, d)
    return head


def search_and_remove(head: Node | None, d: int) -> Node | None:
    ptr = head
    pptr = None
    while ptr is not None:
        if ptr.data == d:
            if pptr is not None:
                pptr.next = ptr.next
            else:
                head = ptr.next
            # "delete ptr": nothing points to it any more
            return head
        pptr = ptr
        ptr = ptr.next
    return head


def remove_head(head: Node | None) -> tuple[Node | None, int | None]:
    """Returns (new head, removed data)."""
    d = None
    if head is not None:
        d = head.data
        head = head.next
    return head, d


def remove_last(head: Node | None) -> tuple[Node | None, int | None]:
    """Returns (new head, removed data)."""
    ptr = head
    pptr = None
    d = None
    if head is not None:
        while ptr.next is not None:
            pptr = ptr
            ptr = ptr.next
        d = ptr.data
        if pptr is not None:
            pptr.next = ptr.next
            return head, d
        else:
            return None, d
    return head, d


def main() -> None:
    head: Node | None = None
    head = init(head)
    for i in range(10):
        print(f"I am adding {i} to list")
        head = insert(head, i)
    head = search_and_remove(head, 0)
    head = search_and_remove(head, 5)
    head = search_and_remove(head, 9)
    while head is not None:
        head, d = remove_head(head)
        print(f"I have removed {d} from list")


if __name__ == "__main__":
    main()
