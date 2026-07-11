import java.util.Arrays;
import java.util.Random;

class main {
    private static int findMissingLinear(int[] values, int maximum) { int remaining = maximum * (maximum + 1) / 2; for (int value : values) remaining -= value; return remaining; }
    private static int findMissingSorted(int[] values) { Arrays.sort(values); for (int i = 0; i < values.length; i++) if (values[i] != i + 1) return i + 1; return values.length + 1; }
    private static int findMissingNonUnique(int[] values) { Arrays.sort(values); int expected = 1; for (int value : values) { if (value > expected) break; if (value == expected) expected++; } return expected; }
    public static void main(String[] args) {
        final int maximum = 250; int missing = new Random().nextInt(maximum) + 1; int[] values = new int[maximum - 1];
        for (int value = 1, index = 0; value <= maximum; value++) if (value != missing) values[index++] = value;
        System.out.println("Missing should be found as: " + missing);
        System.out.println("Missing value is (O(N)): " + findMissingLinear(values, maximum));
        System.out.println("Missing value is (O(nlogn)): " + findMissingSorted(values.clone()));
        int[] duplicates = {1, 1, 2, 2, 4, 4, 5, 5};
        System.out.println("Missing non-unique value is (O(nlogn)): " + findMissingNonUnique(duplicates));
    }
}
