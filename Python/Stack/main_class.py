# A generic stack declaration with a class for any type of data

import sys
from typing import Generic, TypeVar

T = TypeVar("T")


class Stack(Generic[T]):
    def __init__(self) -> None:
        # do nothing for now (except creating the storage)
        self._data: list[T] = []

    def __copy__(self):
        raise TypeError("copy and assignment is forbidden")

    def __deepcopy__(self, memo):
        raise TypeError("copy and assignment is forbidden")

    def is_empty(self) -> bool:
        if len(self._data) == 0:
            return True
        return False

    def is_full(self) -> bool:
        # Python lists have no max_size(); sys.maxsize is the closest limit
        if len(self._data) == sys.maxsize:
            return True
        return False

    def push(self, data: T) -> None:
        if not self.is_full():
            self._data.append(data)
        else:
            print("Stack is full!")

    def pop(self) -> T | None:
        if not self.is_empty():
            temp = self._data[-1]
            self._data.pop()
            return temp
        else:
            print("Stack is empty!")
            return None  # C++ returns nothing here (undefined value)


def main() -> None:
    s: Stack[int] = Stack()
    for i in range(10):
        print(f"I am adding {i} to stack")
        s.push(i)
    while not s.is_empty():
        print(f"I have popped out {s.pop()} from stack")


if __name__ == "__main__":
    main()
