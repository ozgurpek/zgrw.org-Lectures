class ListNode<T> {
  public next?: ListNode<T>;
  public constructor(public data: T) {}
}
class LinkedList<T> {
  private head?: ListNode<T>;
  public insert(data: T): void {
    const node = new ListNode(data);
    if (!this.head) {
      this.head = node;
      return;
    }
    let current = this.head;
    while (current.next) current = current.next;
    current.next = node;
  }
  public searchAndRemove(data: T): void {
    let previous: ListNode<T> | undefined,
      current = this.head;
    while (current) {
      if (current.data === data) {
        if (previous) previous.next = current.next;
        else this.head = current.next;
        return;
      }
      previous = current;
      current = current.next;
    }
  }
  public removeHead(): T | undefined {
    if (!this.head) return undefined;
    const value = this.head.data;
    this.head = this.head.next;
    return value;
  }
  public removeLast(): T | undefined {
    let previous: ListNode<T> | undefined,
      current = this.head;
    if (!current) return undefined;
    while (current.next) {
      previous = current;
      current = current.next;
    }
    if (previous) previous.next = undefined;
    else this.head = undefined;
    return current.data;
  }
}
const list = new LinkedList<number>();
for (let value = 0; value < 10; value++) {
  console.log("I am adding", value, "to list");
  list.insert(value);
}
[0, 5, 9].forEach((value) => list.searchAndRemove(value));
let value: number | undefined;
while ((value = list.removeHead()) !== undefined)
  console.log("I have removed", value, "from list");
