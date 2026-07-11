class main {
    private static class DecimalBigInt {
        private int[] digits;
        DecimalBigInt(String value) {
            digits = new int[value.length()];
            for (int i = 0; i < value.length(); i++) digits[i] = value.charAt(value.length() - 1 - i) - '0';
            trim();
        }
        DecimalBigInt(int value) { this(Integer.toString(value)); }
        DecimalBigInt add(DecimalBigInt other) {
            int[] result = new int[Math.max(digits.length, other.digits.length) + 1];
            for (int i = 0; i < result.length - 1; i++) { int sum = digitAt(i) + other.digitAt(i) + result[i]; result[i] = sum % 10; result[i + 1] = sum / 10; }
            return new DecimalBigInt(result);
        }
        DecimalBigInt subtract(DecimalBigInt other) {
            if (compareTo(other) < 0) return new DecimalBigInt(0);
            int[] result = digits.clone();
            for (int i = 0; i < other.digits.length; i++) { result[i] -= other.digits[i]; if (result[i] < 0) { result[i] += 10; result[i + 1]--; } }
            return new DecimalBigInt(result);
        }
        DecimalBigInt multiply(DecimalBigInt other) {
            int[] result = new int[digits.length + other.digits.length];
            for (int i = 0; i < digits.length; i++) for (int j = 0; j < other.digits.length; j++) result[i + j] += digits[i] * other.digits[j];
            for (int i = 0; i < result.length - 1; i++) { result[i + 1] += result[i] / 10; result[i] %= 10; }
            return new DecimalBigInt(result);
        }
        private DecimalBigInt(int[] digits) { this.digits = digits; trim(); }
        private int digitAt(int index) { return index < digits.length ? digits[index] : 0; }
        private int compareTo(DecimalBigInt other) { if (digits.length != other.digits.length) return digits.length - other.digits.length; for (int i = digits.length - 1; i >= 0; i--) if (digits[i] != other.digits[i]) return digits[i] - other.digits[i]; return 0; }
        private void trim() { int length = digits.length; while (length > 1 && digits[length - 1] == 0) length--; if (length != digits.length) { int[] trimmed = new int[length]; System.arraycopy(digits, 0, trimmed, 0, length); digits = trimmed; } }
        public String toString() { StringBuilder result = new StringBuilder(); for (int i = digits.length - 1; i >= 0; i--) result.append(digits[i]); return result.toString(); }
    }
    public static void main(String[] args) {
        DecimalBigInt first = new DecimalBigInt(9);
        DecimalBigInt second = new DecimalBigInt(3);
        System.out.println(first + " + " + second + " = " + first.add(second));
        System.out.println(first + " - " + second + " = " + first.subtract(second));
        System.out.println(first + " * " + second + " = " + first.multiply(second));
        first = new DecimalBigInt("18446744073709551615");
        System.out.println(first + " + " + second + " = " + first.add(second));
        System.out.println(first + " * " + second + " = " + first.multiply(second));
        DecimalBigInt fifteen = new DecimalBigInt(15);
        System.out.println("15 squared = " + fifteen.multiply(fifteen));
    }
}
