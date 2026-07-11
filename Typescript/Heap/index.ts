class Heap<T> {
  private values: T[] = [];
  public constructor(
    private readonly compare: (first: T, second: T) => number,
  ) {}
  public isEmpty(): boolean {
    return this.values.length === 0;
  }
  public insert(value: T): void {
    this.values.push(value);
    this.bubbleUp(this.values.length - 1);
  }
  public removeMin(): T {
    if (!this.values.length) throw new Error("Heap is empty");
    const minimum = this.values[0],
      last = this.values.pop()!;
    if (this.values.length) {
      this.values[0] = last;
      this.bubbleDown(0);
    }
    return minimum;
  }
  private bubbleUp(index: number): void {
    while (index) {
      const parent = Math.floor((index - 1) / 2);
      if (this.compare(this.values[index], this.values[parent]) >= 0) return;
      this.swap(index, parent);
      index = parent;
    }
  }
  private bubbleDown(index: number): void {
    while (true) {
      const left = index * 2 + 1,
        right = left + 1;
      let smallest = index;
      if (
        left < this.values.length &&
        this.compare(this.values[left], this.values[smallest]) < 0
      )
        smallest = left;
      if (
        right < this.values.length &&
        this.compare(this.values[right], this.values[smallest]) < 0
      )
        smallest = right;
      if (smallest === index) return;
      this.swap(index, smallest);
      index = smallest;
    }
  }
  private swap(first: number, second: number): void {
    [this.values[first], this.values[second]] = [
      this.values[second],
      this.values[first],
    ];
  }
}
const heap = new Heap<number>((first, second) => first - second);
const input = Array.from(
  { length: 11 },
  () => Math.floor(Math.random() * 20) + 1,
);
input.forEach((value) => heap.insert(value));
console.log("Random array:", input.join(" "));
const sorted: number[] = [];
while (!heap.isEmpty()) sorted.push(heap.removeMin());
console.log("Heap sorted array:", sorted.join(" "));
