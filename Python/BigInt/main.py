"""Big integer stored as decimal digits (least significant digit first).

This is a line-by-line port of the C++ version, including its quirks, so that
the console output is the same:
  * the digits live in a fixed list of 1000 ints and `end` is the index of the
    most significant digit (-1 means "no digits", printed as nothing);
  * multiplication does not reset its carry between rows, so big products are
    wrong exactly like in the C++ version;
  * subtraction modifies the left operand while borrowing.
Python ints never overflow, so the C++ `unsigned long long` (ulli) is emulated
with a 64-bit mask where it matters.
"""

ULLI_MAX = 2**64 - 1  # numeric_limits<unsigned long long>::max()


class BigInt:
    def __init__(self, a: int | None = None):
        self.val: list[int] = [0] * 1000
        self.end = -1
        if a is not None:  # explicit bigInt(ulli a)
            self.set(a)

    def __str__(self) -> str:  # operator<<
        out = ""
        i = self.end
        while i >= 0 and self.val[i] >= 0:
            out += str(self.val[i])
            i -= 1
        return out

    def set(self, a: int) -> None:
        self.end = -1
        div = a & ULLI_MAX  # behave like an unsigned 64-bit value
        while div > 0:
            rem = div % 10
            div //= 10
            self.end += 1
            self.val[self.end] = rem

    def assign(self, rhs: "BigInt") -> "BigInt":  # operator=
        self.end = rhs.end
        for i in range(self.end + 1):
            self.val[i] = rhs.val[i]
        return self

    def __add__(self, rhs: "BigInt") -> "BigInt":
        n_val = BigInt()
        mx = max(self.end, rhs.end)
        over = 0
        n_val.end = mx
        for i in range(mx + 1):
            temp = self.val[i] + rhs.val[i] + over
            over = temp // 10  # digits are never negative here, so // matches C++ /
            n_val.val[i] = temp % 10
        if over != 0:
            n_val.end += 1
            n_val.val[n_val.end] = over
        return n_val

    def double_me(self) -> None:
        rhs = self._karatsuba(self, self)
        self.end = rhs.end
        for i in range(self.end + 1):
            self.val[i] = rhs.val[i]

    def _karatsuba(self, lhs: "BigInt", rhs: "BigInt") -> "BigInt":
        # Despite the name this is school-book multiplication (the real
        # Karatsuba code is commented out in the C++ version).
        t1 = BigInt()
        t2 = BigInt()
        t1.end = 0
        t2.end = 0
        over = 0  # note: never reset between rows (same as the C++ code)
        for i in range(self.end + 1):
            t1.end = 0
            j = 0
            while j <= rhs.end:
                a = self.val[i] * rhs.val[j]
                a += over
                over = a // 10
                a %= 10
                t1.val[j] = a
                j += 1
            t1.end = j - 1
            if over != 0:
                t1.val[j] = over
                t1.end += 1
            t1 * i  # shift t1 by i digits (multiply by 10^i)
            s = t2 + t1
            t2.assign(s)
        return t2
        # AB * CD = (B*D) + [(A+B)(C+D) - B*D - A*C]*10^n + A*C*10^2n
        # (the recursive Karatsuba version is left commented out in C++)

    def __mul__(self, rhs: "BigInt | int") -> "BigInt":
        if isinstance(rhs, int):
            # operator*(int): this will only receive powers of 10
            self.end += rhs
            while rhs > 0:
                self.val.insert(0, 0)
                rhs -= 1
            return self
        return self._karatsuba(self, rhs)

    def _partial_copy(self, tar: "BigInt", rhs: "BigInt", beg: int, fin: int) -> None:
        # Only used by the commented-out Karatsuba code
        if fin >= beg:
            end = fin - beg
            for i in range(end + 1):
                tar.val[i] = rhs.val[beg]
                beg += 1

    def get_val(self) -> int:
        val = 0
        if self.end <= 20:
            for i in range(self.end + 1):
                # C++ adds a double (pow returns double) and converts back to ulli
                val = int(float(val) + self.val[i] * 10.0**i) & ULLI_MAX
        return val

    def __sub__(self, rhs: "BigInt") -> "BigInt":
        n_v = BigInt()
        if self.end < rhs.end:
            return BigInt(0)
        for i in range(self.end + 1):
            if self.val[i] < rhs.val[i]:
                if i + 1 > self.end:
                    return BigInt(0)
                self.val[i] += 10
                self.val[i + 1] -= 1
            n_v.val[i] = self.val[i] - rhs.val[i]
        e = self.end
        while e >= 0:
            if n_v.val[e] != 0:
                n_v.end = e
                break
            e -= 1
        return n_v


def main() -> None:
    b1 = BigInt(9)
    b2 = BigInt(3)
    print(f"{b1} + {b2} = {b1 + b2}")
    print(f"{b1} - {b2} = {b1 - b2}")
    print(f"{b1} * {b2} = {b1 * b2}")
    print(b1.get_val())
    b1.set(ULLI_MAX)
    print(b1)
    print(f"{b1} + {b2} = {b1 + b2}")
    print(f"{b1} * {b2} = {b1 * b2}")
    b2.set(15)
    b2.double_me()
    print(b2)


if __name__ == "__main__":
    main()
