from __future__ import annotations

import random

SIZE = 26


class Node:
    def __init__(self) -> None:
        self.val = ""
        self.child: list[Node | None] = [None] * SIZE


def insert_trie(head: Node, txt: str) -> bool:
    curr_node = head
    inserted = False
    for i in range(len(txt)):
        idx = ord(txt[i]) - ord("a")
        if curr_node.child[idx] is None:
            tmp = Node()
            tmp.val = txt[i]
            curr_node.child[idx] = tmp
            curr_node = tmp
            inserted = True
        else:
            curr_node = curr_node.child[idx]
    return inserted


def is_exist(head: Node, txt: str) -> bool:
    curr_node = head
    exists = True
    for i in range(len(txt)):
        idx = ord(txt[i]) - ord("a")
        if curr_node.child[idx] is None:
            exists = False
            break
        curr_node = curr_node.child[idx]
    return exists


def guess_str(head: Node, txt: str) -> list[str]:
    random.seed()  # like srand(time(0))
    curr_node = head
    g: list[str] = []
    if is_exist(head, txt):
        for i in range(len(txt)):
            idx = ord(txt[i]) - ord("a")
            curr_node = curr_node.child[idx]

        # Find all non-null Idx
        idxs: list[int] = []
        for i in range(SIZE):
            if curr_node.child[i] is not None:
                idxs.append(i)

        # Select 3 random idxs
        for k in range(3):
            s = txt
            idx = random.randrange(len(idxs))
            idx = idxs[idx]
            nxt = curr_node.child[idx]
            s += nxt.val
            while nxt is not None:
                found = False
                for i in range(SIZE):
                    # note: nxt changes inside this loop, so the scan continues
                    # in the child node from index i + 1 (same as the C++ code)
                    if nxt.child[i] is not None:
                        nxt = nxt.child[i]
                        found = True
                        s += nxt.val
                if not found:
                    nxt = None
            if s not in g:
                g.append(s)
    return g


def main() -> None:
    head = Node()
    insert_trie(head, "test")
    print(int(is_exist(head, "test")))  # int() so it prints 1 / 0 like C++
    insert_trie(head, "tabak")
    insert_trie(head, "turta")
    insert_trie(head, "tup")
    print(f"Second insert {int(insert_trie(head, 'tup'))}")
    for word in ["araba", "bebek", "ceket", "deniz", "etek", "fasulye", "gezi",
                 "harita", "iplik", "jakuzi", "kalem", "lale", "muzik", "nese",
                 "okul", "parke", "quadro", "rize", "sokak", "tarak", "tarti",
                 "taka", "tasa", "taki", "tatil", "ucak", "veli", "win",
                 "xavier", "yeni", "zeki"]:
        insert_trie(head, word)
    g = guess_str(head, "ta")  # word guesses starting with ta
    for s in g:
        print(s)


if __name__ == "__main__":
    main()
