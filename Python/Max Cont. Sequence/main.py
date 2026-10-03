import random

SIZE = 50


def max_sum(v: list[int]) -> int:
    total = 0
    for val in v:
        if val > 0:
            total += val
    return total


def max_cont_seq(v: list[int]) -> int:  # Kadane's algorithm
    sum_ends_here = 0
    sum_so_far = 0
    for val in v:
        sum_ends_here = max(0, sum_ends_here + val)
        sum_so_far = max(sum_ends_here, sum_so_far)
    return sum_so_far


def main() -> None:
    random.seed()  # like srand(time(0))
    v: list[int] = []
    for i in range(SIZE):
        d = random.randint(0, 15)
        if random.randint(0, 1) == 1:
            d *= -1
        v.append(d)
        print(d, end=" ")
    print()
    print()
    print(f"Max Sum: {max_sum(v)}")
    print(f"Max Cont Sum: {max_cont_seq(v)}")


if __name__ == "__main__":
    main()
