DEF_SIZE = 20


class HashMap:
    """Chained hash map: a list of buckets, each bucket is a Python list."""

    def __init__(self, size: int = DEF_SIZE):  # Default constructor / define size
        self.size = size
        self.v: list[list[int]] = [[] for _ in range(size)]

    def insert(self, data: int) -> None:
        idx = self._hash(data)
        self.v[idx].append(data)

    def remove(self, data: int) -> None:
        idx = self._hash(data)
        # std::list::remove removes every element equal to data
        self.v[idx] = [x for x in self.v[idx] if x != data]

    def search(self, data: int) -> bool:
        idx = self._hash(data)
        return data in self.v[idx]  # check if it is in the list or not and return the result

    def _hash(self, data: int) -> int:
        # Note: for negative numbers Python's % differs from C++ (C++ can return
        # a negative index), but this example only stores positive numbers.
        return data % self.size


def main() -> None:
    hm = HashMap(10)
    for n in [10, 20, 30, 40, 50, 60, 70, 80, 90,
              1, 2, 3, 4, 5, 6, 7, 8, 9,
              11, 22, 33, 44, 55, 66, 77, 88, 99]:
        hm.insert(n)

    # int(...) so that booleans print as 1 / 0 like C++ cout does
    print(f"Search for 10: {int(hm.search(10))}")
    print(f"Search for 19: {int(hm.search(19))}")
    print(f"Search for 99: {int(hm.search(99))}")
    print(f"Search for 9: {int(hm.search(9))}")
    print(f"Search for 20: {int(hm.search(20))}")
    print(f"Search for 101: {int(hm.search(101))}")
    hm.remove(9)
    print(f"Search for 9: {int(hm.search(9))}")


if __name__ == "__main__":
    main()
