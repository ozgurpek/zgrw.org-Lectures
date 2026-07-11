class HashMap<T extends number> {
  private buckets: T[][];
  public constructor(private readonly size = 20) {
    this.buckets = Array.from({ length: size }, () => []);
  }
  public insert(value: T): void {
    this.buckets[this.hash(value)].push(value);
  }
  public search(value: T): boolean {
    return this.buckets[this.hash(value)].indexOf(value) !== -1;
  }
  public remove(value: T): void {
    const bucket = this.buckets[this.hash(value)],
      index = bucket.indexOf(value);
    if (index !== -1) bucket.splice(index, 1);
  }
  private hash(value: T): number {
    return Math.abs(value) % this.size;
  }
}
const map = new HashMap<number>(10);
[
  10, 20, 30, 40, 50, 60, 70, 80, 90, 1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 22, 33, 44,
  55, 66, 77, 88, 99,
].forEach((value) => map.insert(value));
[10, 19, 99, 9, 20, 101].forEach((value) =>
  console.log(`Search for ${value}:`, map.search(value)),
);
map.remove(9);
console.log("Search for 9:", map.search(9));
