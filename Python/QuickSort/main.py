import random


def quick_sort(v: list[int], beg: int = 0, end: int | None = None) -> None:
    """Randomized pivot quick sort, sorts v[beg:end] in place."""
    if end is None:  # quickSort(v) overload: sort the whole list
        end = len(v)
    if end > beg + 1:
        pv = random.randint(beg, end - 1)
        for i in range(beg, end):
            if v[i] < v[pv] and i > pv:
                if i == pv + 1:
                    v[pv], v[i] = v[i], v[pv]
                    pv = i
                else:
                    v[pv], v[pv + 1] = v[pv + 1], v[pv]
                    v[pv], v[i] = v[i], v[pv]
                    pv += 1
            elif v[i] > v[pv] and i < pv:
                v[pv], v[i] = v[i], v[pv]
                pv = i
        quick_sort(v, beg, pv)
        quick_sort(v, pv, end)


def main() -> None:
    random.seed()  # like srand(time(0))
    v: list[int] = []
    print("Array:  ", end="")
    for i in range(20):
        d = random.randint(1, 100)
        v.append(d)
        print(d, end=" ")
    quick_sort(v)
    print()
    print()
    print("Sorted Array:  ", end="")
    for it in v:
        print(it, end=" ")
    print()
    print()


if __name__ == "__main__":
    main()
