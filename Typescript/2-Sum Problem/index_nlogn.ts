function twoSumSorted(
  values: number[],
  target: number,
): Array<[number, number]> {
  values.sort((first, second) => first - second);
  let low = 0,
    high = values.length - 1;
  const pairs: Array<[number, number]> = [];
  while (low < high) {
    const sum = values[low] + values[high];
    if (sum < target) low++;
    else if (sum > target) high--;
    else {
      pairs.push([values[low++], values[high--]]);
    }
  }
  return pairs;
}
const values = Array.from(
    { length: 60 },
    () => Math.floor(Math.random() * 31) - 15,
  ),
  target = 10;
console.log("Array:", values.join(" "));
twoSumSorted(values, target).forEach((pair) =>
  console.log(`${pair[0]} + ${pair[1]} = ${target}`),
);
