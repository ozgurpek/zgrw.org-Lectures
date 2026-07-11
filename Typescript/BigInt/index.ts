class DecimalBigInt {
  private digits: number[];
  public constructor(value: number | string) {
    this.digits = String(value).split("").reverse().map(Number);
    this.trim();
  }
  public add(other: DecimalBigInt): DecimalBigInt {
    const result: number[] = [];
    let carry = 0;
    for (
      let i = 0;
      i < Math.max(this.digits.length, other.digits.length) || carry;
      i++
    ) {
      const total = (this.digits[i] || 0) + (other.digits[i] || 0) + carry;
      result.push(total % 10);
      carry = Math.floor(total / 10);
    }
    return DecimalBigInt.fromDigits(result);
  }
  public subtract(other: DecimalBigInt): DecimalBigInt {
    if (this.compare(other) < 0) return new DecimalBigInt(0);
    const result = this.digits.slice();
    for (let i = 0; i < other.digits.length; i++) {
      result[i] -= other.digits[i];
      if (result[i] < 0) {
        result[i] += 10;
        result[i + 1]--;
      }
    }
    return DecimalBigInt.fromDigits(result);
  }
  public multiply(other: DecimalBigInt): DecimalBigInt {
    const result = new Array(this.digits.length + other.digits.length).fill(0);
    for (let i = 0; i < this.digits.length; i++)
      for (let j = 0; j < other.digits.length; j++)
        result[i + j] += this.digits[i] * other.digits[j];
    for (let i = 0; i < result.length - 1; i++) {
      result[i + 1] += Math.floor(result[i] / 10);
      result[i] %= 10;
    }
    return DecimalBigInt.fromDigits(result);
  }
  public toString(): string {
    return this.digits.slice().reverse().join("");
  }
  private compare(other: DecimalBigInt): number {
    if (this.digits.length !== other.digits.length)
      return this.digits.length - other.digits.length;
    for (let i = this.digits.length - 1; i >= 0; i--)
      if (this.digits[i] !== other.digits[i])
        return this.digits[i] - other.digits[i];
    return 0;
  }
  private trim(): void {
    while (this.digits.length > 1 && this.digits[this.digits.length - 1] === 0)
      this.digits.pop();
  }
  private static fromDigits(digits: number[]): DecimalBigInt {
    const result = new DecimalBigInt(0);
    result.digits = digits;
    result.trim();
    return result;
  }
}
const first = new DecimalBigInt(9),
  second = new DecimalBigInt(3),
  maximum = new DecimalBigInt("18446744073709551615");
console.log(`${first} + ${second} = ${first.add(second)}`);
console.log(`${first} - ${second} = ${first.subtract(second)}`);
console.log(`${first} * ${second} = ${first.multiply(second)}`);
console.log(`${maximum} + ${second} = ${maximum.add(second)}`);
console.log(`${maximum} * ${second} = ${maximum.multiply(second)}`);
