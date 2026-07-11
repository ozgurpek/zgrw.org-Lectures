function merge(left: number[], right: number[]): number[] {
  const result: number[] = [];
  let i = 0,
    j = 0;
  while (i < left.length && j < right.length)
    result.push(left[i] <= right[j] ? left[i++] : right[j++]);
  return result.concat(left.slice(i), right.slice(j));
}

function mergeSort(values: number[]): number[] {
  if (values.length <= 1) return values;
  const middle = Math.floor(values.length / 2);
  return merge(
    mergeSort(values.slice(0, middle)),
    mergeSort(values.slice(middle)),
  );
}

const values = Array.from(
  { length: 25 },
  () => Math.floor(Math.random() * 100) + 1,
);
console.log("Array:", values.join(" "));
console.log("Sorted Array:", mergeSort(values).join(" "));
