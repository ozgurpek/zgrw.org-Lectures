function twoSum(values: number[], target: number): Array<[number, number]> {
  const seen: { [key: number]: boolean } = {},
    pairs: Array<[number, number]> = [];
  values.forEach((value) => {
    const remainder = target - value;
    if (seen[remainder]) pairs.push([value, remainder]);
    seen[value] = true;
  });
  return pairs;
}
const values = Array.from(
    { length: 60 },
    () => Math.floor(Math.random() * 31) - 15,
  ),
  target = 10;
console.log("Array:", values.join(" "));
console.log(`Pairs summing to ${target}:`);
twoSum(values, target).forEach((pair) =>
  console.log(`${pair[0]} + ${pair[1]} = ${target}`),
);
