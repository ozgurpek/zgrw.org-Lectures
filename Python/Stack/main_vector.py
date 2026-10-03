"""Stack on top of a growable list (std::vector in C++), with free functions."""

import sys

# MAX_SIZE = 10


class Stack:  # "struct stack" in the C++ version
    def __init__(self) -> None:
        self.data: list[int] = []
        # self.data = [0] * MAX_SIZE
        # self.ptr = 0


# def init(s: Stack) -> None:
#     s.ptr = 0


def is_empty(s: Stack) -> bool:
    if len(s.data) == 0:
        return True
    return False


def is_full(s: Stack) -> bool:
    # Python lists have no max_size(); sys.maxsize is the closest limit
    if len(s.data) == sys.maxsize:
        return True
    return False


def push(s: Stack, data: int) -> None:
    if not is_full(s):
        s.data.append(data)
    else:
        print("Stack is full!")


def pop(s: Stack) -> int | None:
    if not is_empty(s):
        temp = s.data[-1]
        s.data.pop()
        return temp
    else:
        print("Stack is empty!")
        return None  # C++ returns nothing here (undefined value)


def main() -> None:
    s = Stack()
    for i in range(10):
        print(f"I am adding {i} to stack")
        push(s, i)
    while not is_empty(s):
        print(f"I have popped out {pop(s)} from stack")


if __name__ == "__main__":
    main()
