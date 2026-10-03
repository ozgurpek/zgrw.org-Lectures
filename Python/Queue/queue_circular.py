"""Circular queue with free functions (C-style), like queue_circular.cc.

A fixed-size Python list stands in for the C++ `int data[SIZE]` array.
"""

SIZE = 10


class Queue:  # "struct queue" in the C++ version
    def __init__(self) -> None:
        self.beg = -1
        self.end = -1
        self.data = [0] * SIZE


def init(q: Queue) -> None:
    q.beg = -1
    q.end = -1


def is_full(q: Queue) -> bool:
    if (q.end == SIZE - 1 and q.beg == 0) or (q.end + 1 == q.beg and q.end != -1):
        return True
    return False


def is_empty(q: Queue) -> bool:
    return q.beg == -1


def insert(q: Queue, d: int) -> None:
    if not is_full(q):
        if is_empty(q):
            q.beg = q.end
        q.end += 1
        if (q.end == SIZE and q.beg == 0) or q.end == q.beg:
            q.end -= 1
        elif q.end == SIZE and q.beg != 0:
            q.end = 0
        q.data[q.end] = d


def remove(q: Queue) -> int:
    if not is_empty(q):
        d = q.data[q.beg]
        q.beg += 1
        if q.beg == q.end + 1:
            q.beg = -1
        elif q.beg == SIZE:
            q.beg = 0
        return d
    return -1


def main() -> None:
    q = Queue()
    init(q)
    i = 0
    while not is_full(q):
        print(f"I am adding {i} to queue")
        insert(q, i)
        i += 1
    while not is_empty(q):
        print(f"I have removed {remove(q)} from queue")


if __name__ == "__main__":
    main()
