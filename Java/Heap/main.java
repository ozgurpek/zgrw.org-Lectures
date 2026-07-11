import java.util.ArrayList;
import java.util.NoSuchElementException;
import java.util.Random;

class main {
    private static class MinHeap<T extends Comparable<T>> {
        private final ArrayList<T> values = new ArrayList<T>();
        boolean isEmpty() { return values.isEmpty(); }
        void insert(T value) { values.add(value); bubbleUp(values.size() - 1); }
        T removeMin() {
            if (values.isEmpty()) throw new NoSuchElementException("Heap is empty");
            T minimum = values.get(0);
            T last = values.remove(values.size() - 1);
            if (!values.isEmpty()) { values.set(0, last); bubbleDown(0); }
            return minimum;
        }
        private void bubbleUp(int index) {
            while (index > 0) {
                int parent = (index - 1) / 2;
                if (values.get(index).compareTo(values.get(parent)) >= 0) return;
                swap(index, parent); index = parent;
            }
        }
        private void bubbleDown(int index) {
            while (true) {
                int left = index * 2 + 1, right = left + 1, smallest = index;
                if (left < values.size() && values.get(left).compareTo(values.get(smallest)) < 0) smallest = left;
                if (right < values.size() && values.get(right).compareTo(values.get(smallest)) < 0) smallest = right;
                if (smallest == index) return;
                swap(index, smallest); index = smallest;
            }
        }
        private void swap(int first, int second) { T value = values.get(first); values.set(first, values.get(second)); values.set(second, value); }
    }

    public static void main(String[] args) {
        MinHeap<Integer> heap = new MinHeap<Integer>();
        Random random = new Random();
        System.out.println("Random array:");
        for (int i = 0; i < 11; i++) { int value = random.nextInt(20) + 1; heap.insert(value); System.out.print(value + " "); }
        System.out.println("\n\nHeap sorted array:");
        while (!heap.isEmpty()) System.out.println(heap.removeMin());
    }
}
