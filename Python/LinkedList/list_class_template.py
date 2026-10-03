"""Generic singly linked list (template version), like list_class_template.cc.

Python is dynamically typed, so a "template" is just a class that works with
any data type; Generic[T] only documents that for type checkers.
Every method returns the (possibly new) head of the list, as in C++.
The C++ `T &d` out-parameters become extra return values.
"""

from __future__ import annotations

from typing import Generic, TypeVar

T = TypeVar("T")


class List(Generic[T]):
    def __init__(self, data: T) -> None:
        # In C++ the constructor is private and only init() creates nodes
        self.next: List[T] | None = None
        self.data = data

    def insert(self, d: T) -> List[T]:
        if self.next is None:
            self.next = init(self.next, d)
        else:
            self.next = self.next.insert(d)
        return self

    def search_and_remove(self, d: T) -> List[T] | None:
        ptr: List[T] | None = self
        pptr: List[T] | None = None
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

    def remove_head(self) -> tuple[List[T] | None, T]:
        """Returns (new head, removed data)."""
        d = self.data
        ptr = self.next
        return ptr, d

    def remove_last(self) -> tuple[List[T] | None, T]:
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


def init(head: List[T] | None, d: T) -> List[T] | None:  # friend function in C++
    if head is None:
        head = List(d)
        head.next = None
    return head


def main() -> None:
    head: List[float] | None = None
    head = init(head, 0.0)
    i = 1.0
    while i < 10:
        # :g prints doubles the way C++ cout does (1 instead of 1.0)
        print(f"I am adding {i:g} to list")
        head = head.insert(i)
        i += 1
    head = head.search_and_remove(0.0)
    head = head.search_and_remove(5.0)
    head = head.search_and_remove(9.0)
    while head is not None:
        head, d = head.remove_head()
        print(f"I have removed {d:g} from list")


if __name__ == "__main__":
    main()
