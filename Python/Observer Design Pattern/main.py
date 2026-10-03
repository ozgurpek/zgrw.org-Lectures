from classes import DivObserver, ModObserver, Subject


def main() -> None:
    s = Subject()
    d1 = DivObserver(s, 3)
    m1 = ModObserver(s, 7)
    d2 = DivObserver(s, 4)
    m2 = ModObserver(s, 5)
    s.set_val(10)


if __name__ == "__main__":
    main()
