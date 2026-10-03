Vector = list[int]
Matrix = list[list[int]]


def dot(v1: Vector, v2: Vector) -> int:
    """Dot product (operator* for two vectors in the C++ version)."""
    total = 0
    sz = min(len(v1), len(v2))  # remaining dimensions for the smaller vector will be accepted as 0 as we were in the bigger dimension world
    for i in range(sz):
        total += v1[i] * v2[i]
    return total


def mat_mul(m1: Matrix, m2: Matrix) -> Matrix:
    """Matrix multiplication (operator* for two matrices in the C++ version)."""
    sz1 = len(m1)
    sz2 = len(m2)
    res: Matrix = []
    if sz1 == len(m2[0]) and sz2 == len(m1[0]):
        for i in range(sz1):
            row: Vector = []
            for k in range(sz1):
                total = 0
                for j in range(sz2):
                    total += m1[i][j] * m2[j][k]
                row.append(total)
            res.append(row)
    return res


def matrix_to_str(m1: Matrix) -> str:
    """Same text as operator<< for a matrix: each value followed by a space, one row per line."""
    out = ""
    for ml in m1:
        for ms in ml:
            out += f"{ms} "
        out += "\n"
    return out


def main() -> None:
    v1 = [1, 2, 3]
    v2 = [3, 2, 1]
    print(f"v1 * v2 = {dot(v1, v2)}")
    m1 = [[1, 2, 3], [4, 5, 6]]
    m2 = [[1, 2], [3, 4], [5, 6]]
    print("m1 * m2; ")
    print(matrix_to_str(mat_mul(m1, m2)))


if __name__ == "__main__":
    main()
