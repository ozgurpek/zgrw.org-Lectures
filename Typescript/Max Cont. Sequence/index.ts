function maxSum(values: number[]): number {
  return values
    .filter((value) => value > 0)
    .reduce((sum, value) => sum + value, 0);
}
function maxContiguousSequence(values: number[]): number {
  let endingHere = 0,
    best = 0;
  values.forEach((value) => {
    endingHere = Math.max(0, endingHere + value);
    best = Math.max(best, endingHere);
  });
  return best;
}
const values = Array.from(
  { length: 50 },
  () => Math.floor(Math.random() * 16) * (Math.random() < 0.5 ? -1 : 1),
);
console.log(values.join(" "));
console.log("Max Sum:", maxSum(values));
console.log("Max Cont Sum:", maxContiguousSequence(values));
