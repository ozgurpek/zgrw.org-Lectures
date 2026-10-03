import random

DEF_SIZE = 20


class HashMap:
    """Chained hash map: a list of buckets, each bucket is a Python list."""

    def __init__(self, size: int = DEF_SIZE):  # Default constructor / define size
        self.size = size
        self.v: list[list[int]] = [[] for _ in range(size)]

    def insert(self, data: int) -> None:
        idx = self._hash(data)
        self.v[idx].append(data)

    def search(self, data: int) -> bool:
        idx = self._hash(data)
        res = data in self.v[idx]  # check if it is in the list or not and return the result
        self.remove(data)
        return res

    def remove(self, data: int) -> None:
        idx = self._hash(data)
        # std::list::remove removes every element equal to data
        self.v[idx] = [x for x in self.v[idx] if x != data]

    def _hash(self, data: int) -> int:
        return abs(data) % self.size


def main() -> None:
    random.seed()  # like srand(time(0))
    v: list[int] = []
    for i in range(DEF_SIZE * 3):
        d = random.randint(0, 15)
        if random.randint(0, 1) == 1:
            d *= -1
        v.append(d)
        print(d, end=" ")
    print()
    print()
    print("Enter a number between 1 - 15 as sum: ", end="", flush=True)
    target = int(input())
    hm = HashMap()
    for it in v:
        rm = target - it
        if hm.search(rm):
            print(f"{it} - {rm}")
        hm.insert(it)


if __name__ == "__main__":
    main()
