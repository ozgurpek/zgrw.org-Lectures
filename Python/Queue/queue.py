"""Array based queue with free functions (C-style), like queue.cc.

A fixed-size Python list stands in for the C++ `int data[SIZE]` array.
"""

SIZE = 10


class Queue:  # "struct queue" in the C++ version
    def __init__(self) -> None:
        self.end = 0
        self.data = [0] * SIZE


def init(q: Queue) -> None:
    q.end = 0


def is_full(q: Queue) -> bool:
    return q.end == SIZE


def is_empty(q: Queue) -> bool:
    return q.end == 0


def insert(q: Queue, d: int) -> None:
    if not is_full(q):
        q.data[q.end] = d
        q.end += 1


def shift_queue(q: Queue) -> None:
    for i in range(q.end - 1):
        q.data[i] = q.data[i + 1]
    q.end -= 1


def remove(q: Queue) -> int:
    if not is_empty(q):
        d = q.data[0]
        shift_queue(q)
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
