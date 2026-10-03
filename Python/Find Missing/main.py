import random

MAX = 250


def find_missing(vec: list[int], max_val: int) -> int:  # O(N) solution
    remain = (max_val * (max_val + 1)) // 2
    for i in range(len(vec)):  # O(n) loop
        remain -= vec[i]
    return remain


def find_missing_sorted(vec: list[int]) -> int:  # O(nlogn) solution
    vec.sort()  # O(nlogn) sort
    i = 0
    while i < len(vec):  # O(n) loop
        if vec[i] != i + 1:
            break
        i += 1
    return i + 1  # because i starts from 0


def find_missing_non_unique(vec: list[int]) -> int:  # O(nlogn) solution
    vec.sort()  # O(nlogn) sort
    look_up = 1
    for i in range(len(vec)):  # O(n) loop
        if vec[i] > look_up:
            break
        elif vec[i] == look_up:
            look_up += 1
    return look_up


def main() -> None:
    random.seed()  # like srand(time(0))
    missing = random.randint(1, MAX)
    print(f"Missing should be found as: {missing}")
    v: list[int] = []
    print("Array is: ", end="")
    for i in range(1, MAX + 1):
        if i != missing:
            v.append(i)
            print(i, end=" ")
    print()
    print()
    print(f"Missing value is (O(N)): {find_missing(v, MAX)}")
    print(f"Missing value is (O(nlogn)): {find_missing_sorted(v)}")
    v.clear()
    print()
    print()
    print("Non-unique array is: ", end="")
    for i in range(1, MAX + 1):
        d = random.randint(1, 3)  # max three occurences
        j = 0
        while j < d and i != missing:
            v.append(i)
            print(i, end=" ")
            j += 1
    print()
    print()
    print(f"Missing non-unique value is (O(nlogn)): {find_missing_non_unique(v)}")


if __name__ == "__main__":
    main()
