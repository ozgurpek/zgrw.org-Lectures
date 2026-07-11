function quickSort(values: number[], begin = 0, end = values.length): void {
  if (end <= begin + 1) return;
  const pivotIndex = Math.floor(Math.random() * (end - begin)) + begin;
  [values[pivotIndex], values[end - 1]] = [values[end - 1], values[pivotIndex]];
  let boundary = begin;
  for (let i = begin; i < end - 1; i++)
    if (values[i] < values[end - 1])
      [values[i], values[boundary]] = [values[boundary++], values[i]];
  [values[boundary], values[end - 1]] = [values[end - 1], values[boundary]];
  quickSort(values, begin, boundary);
  quickSort(values, boundary + 1, end);
}

const values = Array.from(
  { length: 20 },
  () => Math.floor(Math.random() * 100) + 1,
);
console.log("Array:", values.join(" "));
quickSort(values);
console.log("Sorted Array:", values.join(" "));
