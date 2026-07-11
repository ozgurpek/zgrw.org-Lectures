class main {
    private static int dotProduct(int[] first, int[] second) {
        int sum = 0;
        for (int i = 0; i < Math.min(first.length, second.length); i++) sum += first[i] * second[i];
        return sum;
    }

    private static int[][] multiply(int[][] first, int[][] second) {
        if (first[0].length != second.length) throw new IllegalArgumentException("Incompatible matrices");
        int[][] result = new int[first.length][second[0].length];
        for (int row = 0; row < first.length; row++)
            for (int column = 0; column < second[0].length; column++)
                for (int index = 0; index < second.length; index++) result[row][column] += first[row][index] * second[index][column];
        return result;
    }

    private static void print(int[][] matrix) {
        for (int[] row : matrix) {
            for (int value : row) System.out.print(value + " ");
            System.out.println();
        }
    }

    public static void main(String[] args) {
        System.out.println("v1 * v2 = " + dotProduct(new int[] {1, 2, 3}, new int[] {3, 2, 1}));
        print(multiply(new int[][] {{1, 2, 3}, {4, 5, 6}}, new int[][] {{1, 2}, {3, 4}, {5, 6}}));
    }
}
