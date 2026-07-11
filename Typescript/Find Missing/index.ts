function findMissingLinear(values: number[], maximum: number): number {
  return (
    (maximum * (maximum + 1)) / 2 -
    values.reduce((sum, value) => sum + value, 0)
  );
}
function findMissingSorted(values: number[]): number {
  values.sort((first, second) => first - second);
  for (let i = 0; i < values.length; i++) if (values[i] !== i + 1) return i + 1;
  return values.length + 1;
}
function findMissingNonUnique(values: number[]): number {
  values.sort((first, second) => first - second);
  let expected = 1;
  values.forEach((value) => {
    if (value === expected) expected++;
  });
  return expected;
}
const maximum = 250,
  missing = Math.floor(Math.random() * maximum) + 1;
const values = Array.from({ length: maximum }, (_, i) => i + 1).filter(
  (value) => value !== missing,
);
console.log("Missing should be found as:", missing);
console.log("Missing value is (O(N)):", findMissingLinear(values, maximum));
console.log("Missing value is (O(nlogn)):", findMissingSorted(values.slice()));
console.log(
  "Missing non-unique value is (O(nlogn)):",
  findMissingNonUnique([1, 1, 2, 2, 4, 4, 5]),
);
