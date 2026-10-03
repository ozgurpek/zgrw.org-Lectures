"""Generic circular queue as a class (template version), like queue_circular_class.cc.

A fixed-size Python list stands in for the C++ `T data[SIZE]` array.
"""

from typing import Generic, TypeVar

SIZE = 10

T = TypeVar("T")


class Queue(Generic[T]):
    def __init__(self) -> None:
        self._beg = -1
        self._end = -1
        self._data: list = [None] * SIZE

    def is_full(self) -> bool:
        if (self._end == SIZE - 1 and self._beg == 0) or (self._end + 1 == self._beg and self._end != -1):
            return True
        return False

    def is_empty(self) -> bool:
        return self._beg == -1

    def insert(self, d: T) -> None:
        if not self.is_full():
            if self.is_empty():
                self._beg = self._end
            self._end += 1
            if (self._end == SIZE and self._beg == 0) or self._end == self._beg:
                self._end -= 1
            elif self._end == SIZE and self._beg != 0:
                self._end = 0
            self._data[self._end] = d

    def remove(self) -> T:
        if not self.is_empty():
            d = self._data[self._beg]
            self._beg += 1
            if self._beg == self._end + 1:
                self._beg = -1
            elif self._beg == SIZE:
                self._beg = 0
            return d
        return -1


def main() -> None:
    q: Queue[int] = Queue()
    i = 0
    while not q.is_full():
        print(f"I am adding {i} to queue")
        q.insert(i)
        i += 1
    while not q.is_empty():
        print(f"I have removed {q.remove()} from queue")


if __name__ == "__main__":
    main()
