import random


def merge(p1: list[int], p2: list[int], nend: int, end: int) -> list[int]:
    i = 0
    j = 0
    vn: list[int] = []
    while i < nend and j < end - nend:
        if p1[i] > p2[j]:
            vn.append(p2[j])
            j += 1
        else:
            vn.append(p1[i])
            i += 1
    while i < nend:  # if not finished
        vn.append(p1[i])
        i += 1
    while j < end - nend:  # because second array might be bigger
        vn.append(p2[j])
        j += 1
    return vn


def merge_sort(v: list[int]) -> list[int]:
    if len(v) > 1:
        nend = len(v) // 2
        end = len(v)
        p1: list[int] = []
        p2: list[int] = []
        for i in range(nend):
            p1.append(v[i])
        for i in range(nend, end):
            p2.append(v[i])

        p1 = merge_sort(p1)
        p2 = merge_sort(p2)

        return merge(p1, p2, nend, end)
    else:
        return v


def main() -> None:
    random.seed()  # like srand(time(0))
    v: list[int] = []
    print("Array:  ", end="")
    v.append(23)  # note: this first value is sorted but not printed (same as C++)
    for i in range(25):
        d = random.randint(1, 100)
        v.append(d)
        print(d, end=" ")
    v = merge_sort(v)
    print()
    print()
    print("Sorted Array:  ", end="")
    for it in v:
        print(it, end=" ")
    print()
    print()


if __name__ == "__main__":
    main()
