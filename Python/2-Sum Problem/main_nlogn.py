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
    for i in range(60):
        d = random.randint(1, 16)
        if random.randint(0, 1) == 1:
            d *= -1
        v.append(d)
        print(d, end=" ")
    quick_sort(v)

    print()
    print()
    print("Enter a number between 1 - 15 as sum: ", end="", flush=True)
    target = int(input())

    # Two "iterators" (indexes): one from the end, one from the beginning
    itu = len(v) - 1
    itl = 0
    while itl < len(v) and itu >= 0 and itl < itu:
        temp = v[itl] + v[itu]
        if temp < target:
            itl += 1
        elif temp > target:
            itu -= 1
        else:
            print(f"{v[itl]} - {v[itu]}")
            itl += 1
            itu -= 1


if __name__ == "__main__":
    main()
