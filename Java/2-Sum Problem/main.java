import java.util.Arrays;
import java.util.HashSet;
import java.util.Random;
import java.util.Set;

class main {
    private static void twoSum(int[] values, int target) {
        Set<Integer> seen = new HashSet<Integer>();
        for (int value : values) { int remainder = target - value; if (seen.contains(remainder)) System.out.println(value + " + " + remainder + " = " + target); seen.add(value); }
    }
    private static void twoSumSorted(int[] values, int target) {
        Arrays.sort(values);
        for (int low = 0, high = values.length - 1; low < high;) {
            int sum = values[low] + values[high];
            if (sum < target) low++;
            else if (sum > target) high--;
            else { System.out.println(values[low++] + " + " + values[high--] + " = " + target); }
        }
    }
    public static void main(String[] args) {
        int[] values = new int[60]; Random random = new Random();
        System.out.print("Array: ");
        for (int i = 0; i < values.length; i++) { values[i] = random.nextInt(31) - 15; System.out.print(values[i] + " "); }
        System.out.println("\n\nPairs summing to 10 (hash map):"); twoSum(values, 10);
        System.out.println("Pairs summing to 10 (sorted):"); twoSumSorted(values.clone(), 10);
    }
}
