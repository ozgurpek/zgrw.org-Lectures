"""Array based stack with free functions (C-style), like main.cc.

A fixed-size Python list stands in for the C++ `int data[MAX_SIZE]` array.
"""

MAX_SIZE = 10


class Stack:  # "struct stack" in the C++ version
    def __init__(self) -> None:
        self.data = [0] * MAX_SIZE
        self.ptr = 0


def init(s: Stack) -> None:
    s.ptr = 0


def is_empty(s: Stack) -> bool:
    if s.ptr == 0:
        return True
    return False


def is_full(s: Stack) -> bool:
    if s.ptr == MAX_SIZE:
        return True
    return False


def push(s: Stack, data: int) -> None:
    if not is_full(s):
        s.data[s.ptr] = data
        s.ptr += 1
    else:
        print("Stack is full!")


def pop(s: Stack) -> int | None:
    if not is_empty(s):
        s.ptr -= 1
        return s.data[s.ptr]
    else:
        print("Stack is empty!")
        return None  # C++ returns nothing here (undefined value)


def main() -> None:
    s = Stack()
    init(s)
    i = 0
    while not is_full(s):
        print(f"I am adding {i} to stack")
        push(s, i)
        i += 1
    while not is_empty(s):
        print(f"I have popped out {pop(s)} from stack")


if __name__ == "__main__":
    main()
