import random


class Heap:  # min heap implementation
    def __init__(self):
        self.v: list[int] = []

    def is_empty(self) -> bool:
        return len(self.v) == 0

    def insert(self, data: int) -> None:
        idx = len(self.v)
        self.v.append(data)
        self._heapify(idx)

    def get_min(self) -> int:
        return self.v[0]

    def _heapify(self, idx: int) -> None:
        if idx != 0:  # combo breaker
            parrent = idx // 2 if idx % 2 == 0 else (idx - 1) // 2
            if self.v[idx] < self.v[parrent]:
                self.v[idx], self.v[parrent] = self.v[parrent], self.v[idx]
                self._heapify(parrent)

    def remove_min(self) -> int:
        idx = len(self.v) - 1
        self.v[idx], self.v[0] = self.v[0], self.v[idx]
        val = self.v[idx]
        self.v.pop()
        self._heapify_del(0)
        return val

    def _heapify_del(self, idx: int) -> None:
        # C++: idx <= ceil((v.size() - 1) / 2) - 1  -- the division is already
        # an integer division there, so ceil() changes nothing.
        if idx <= (len(self.v) - 1) // 2 - 1 and len(self.v) > 1:
            ch1 = 1 if idx == 0 else idx * 2
            ch2 = ch1 + 1

            if self.v[ch1] < self.v[ch2]:
                m_idx = ch1
            else:
                m_idx = ch2

            if self.v[idx] > self.v[m_idx]:
                self.v[idx], self.v[m_idx] = self.v[m_idx], self.v[idx]
                self._heapify_del(m_idx)


def main() -> None:
    random.seed()  # like srand(time(0))
    h = Heap()
    print("Random array;")
    for i in range(11):
        d = random.randint(1, 20)
        h.insert(d)
        print(d, end=" ")
    print()
    print()
    print("Heap sorted array;")
    while not h.is_empty():  # Here an example of heap sort :)
        print(h.remove_min())


if __name__ == "__main__":
    main()
