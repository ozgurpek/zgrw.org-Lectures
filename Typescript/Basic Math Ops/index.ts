function dotProduct(first: number[], second: number[]): number {
  return first
    .slice(0, second.length)
    .reduce((sum, value, i) => sum + value * second[i], 0);
}
function multiply(first: number[][], second: number[][]): number[][] {
  if (first[0].length !== second.length)
    throw new Error("Incompatible matrices");
  return first.map((row) =>
    second[0].map((_, column) =>
      row.reduce((sum, value, index) => sum + value * second[index][column], 0),
    ),
  );
}
console.log("v1 * v2 =", dotProduct([1, 2, 3], [3, 2, 1]));
console.log(
  multiply(
    [
      [1, 2, 3],
      [4, 5, 6],
    ],
    [
      [1, 2],
      [3, 4],
      [5, 6],
    ],
  )
    .map((row) => row.join(" "))
    .join("\n"),
);
