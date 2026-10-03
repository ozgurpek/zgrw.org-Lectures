"""Singly linked list as a class, like list_class.cc.

Every method returns the (possibly new) head of the list, as in C++.
Python has no `delete`: removed nodes are freed by the garbage collector.
The C++ `int &d` out-parameters become extra return values.
"""

from __future__ import annotations


class List:
    def __init__(self) -> None:
        # In C++ the constructor is private and only init() creates nodes
        self.next: List | None = None
        self.data = 0

    def insert(self, d: int) -> List:
        # (the C++ `this != NULL` check is not needed: self is never None)
        if self.next is None:
            self.next = init(self.next, d)
        else:
            self.next = self.next.insert(d)
        return self

    def search_and_remove(self, d: int) -> List | None:
        ptr: List | None = self
        pptr: List | None = None
        while ptr is not None:
            if ptr.data == d:
                if pptr is not None:
                    pptr.next = ptr.next
                    return self
                else:
                    # removing the head: the next node becomes the head
                    return ptr.next
            pptr = ptr
            ptr = ptr.next
        return self

    def remove_head(self) -> tuple[List | None, int]:
        """Returns (new head, removed data)."""
        d = self.data
        ptr = self.next
        return ptr, d

    def remove_last(self) -> tuple[List | None, int]:
        """Returns (new head, removed data)."""
        ptr = self
        pptr = None
        while ptr.next is not None:
            pptr = ptr
            ptr = ptr.next
        d = ptr.data
        if pptr is not None:
            pptr.next = ptr.next
            return self, d
        else:
            return None, d


def init(head: List | None, d: int) -> List | None:  # friend function in C++
    if head is None:
        head = List()
        head.data = d
        head.next = None
    return head


def main() -> None:
    head: List | None = None
    head = init(head, 0)
    for i in range(1, 10):
        print(f"I am adding {i} to list")
        head = head.insert(i)
    head = head.search_and_remove(0)
    head = head.search_and_remove(5)
    head = head.search_and_remove(9)
    while head is not None:
        head, d = head.remove_head()
        print(f"I have removed {d} from list")


if __name__ == "__main__":
    main()
