import java.util.ArrayList;
import java.util.LinkedList;
import java.util.List;

class main {
    private static class HashSet<T> {
        private final List<LinkedList<T>> buckets;
        HashSet(int size) { buckets = new ArrayList<LinkedList<T>>(size); for (int i = 0; i < size; i++) buckets.add(new LinkedList<T>()); }
        void insert(T value) { bucket(value).add(value); }
        boolean search(T value) { return bucket(value).contains(value); }
        void remove(T value) { bucket(value).remove(value); }
        private LinkedList<T> bucket(T value) { return buckets.get(Math.floorMod(value.hashCode(), buckets.size())); }
    }
    public static void main(String[] args) {
        HashSet<Integer> map = new HashSet<Integer>(10);
        for (int value : new int[] {10, 20, 30, 40, 50, 60, 70, 80, 90, 1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 22, 33, 44, 55, 66, 77, 88, 99}) map.insert(value);
        for (int value : new int[] {10, 19, 99, 9, 20, 101}) System.out.println("Search for " + value + ": " + map.search(value));
        map.remove(9); System.out.println("Search for 9: " + map.search(9));
    }
}
