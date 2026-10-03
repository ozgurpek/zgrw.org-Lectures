use std::fmt;

type Ulli = u64; // unsigned long long int

// Digits are stored least significant first: val[0] is the ones digit.
// The C++ operators become named methods here (add, sub, mul, assign, ...)
// because they take `&mut` / `&` exactly like the C++ version does.
struct BigInt {
    val: Vec<i32>,
    end: i32, // index of the most significant digit, -1 means "no digits"
}

// operator<<
impl fmt::Display for BigInt {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        let mut i = self.end;
        while i >= 0 && self.val[i as usize] >= 0 {
            write!(out, "{}", self.val[i as usize])?;
            i -= 1;
        }
        Ok(())
    }
}

impl BigInt {
    fn new() -> BigInt {
        BigInt { val: vec![0; 1000], end: -1 }
    }

    fn from(a: Ulli) -> BigInt {
        let mut b = BigInt::new();
        b.set(a);
        b
    }

    fn set(&mut self, a: Ulli) {
        self.end = -1;
        let mut div = a;
        while div > 0 {
            let rem = (div % 10) as i32;
            div /= 10;
            self.end += 1;
            self.val[self.end as usize] = rem;
        }
    }

    // operator=
    fn assign(&mut self, rhs: &BigInt) {
        self.end = rhs.end;
        for i in 0..=self.end {
            self.val[i as usize] = rhs.val[i as usize];
        }
    }

    // operator+
    fn add(&self, rhs: &BigInt) -> BigInt {
        let mut n_val = BigInt::new();
        let mx = self.end.max(rhs.end);
        let mut over = 0;
        n_val.end = mx;
        for i in 0..=mx {
            let i = i as usize;
            let temp = self.val[i] + rhs.val[i] + over;
            over = temp / 10;
            n_val.val[i] = temp % 10;
        }
        if over != 0 {
            n_val.end += 1;
            n_val.val[n_val.end as usize] = over;
        }
        n_val
    }

    fn double_me(&mut self) {
        let rhs = self.karatsuba(self, self);
        self.end = rhs.end;
        for i in 0..=self.end {
            self.val[i as usize] = rhs.val[i as usize];
        }
    }

    // Named karatsuba, but (as in the C++ version) it is really the
    // schoolbook method using `self` and `rhs`; `_lhs` is not used.
    fn karatsuba(&self, _lhs: &BigInt, rhs: &BigInt) -> BigInt {
        let mut t1 = BigInt::new();
        let mut t2 = BigInt::new();
        t1.end = 0;
        t2.end = 0;
        let mut over = 0;
        for i in 0..=self.end {
            t1.end = 0;
            let mut j = 0;
            while j <= rhs.end {
                let mut a = self.val[i as usize] * rhs.val[j as usize];
                a += over;
                over = a / 10;
                a %= 10;
                t1.val[j as usize] = a;
                j += 1;
            }
            t1.end = j - 1;
            if over != 0 {
                t1.val[j as usize] = over;
                t1.end += 1;
            }
            t1.mul_pow10(i);
            let s = t2.add(&t1);
            t2.assign(&s);
        }
        t2
        //AB * CD = (B*D) + [(A+B)(C+D) - B*D - A*C]*10^n + A*C*10^2n
        // (the recursive Karatsuba version is commented out in the C++ file too)
    }

    // operator*(bigInt)
    fn mul(&self, rhs: &BigInt) -> BigInt {
        self.karatsuba(self, rhs)
    }

    // operator*(int): this will only recive powers of 10
    fn mul_pow10(&mut self, mut rhs: i32) {
        self.end += rhs;
        while rhs > 0 {
            self.val.insert(0, 0);
            rhs -= 1;
        }
    }

    #[allow(dead_code)] // only used by the commented out Karatsuba code, kept for parity
    fn partial_copy(&self, tar: &mut BigInt, rhs: &BigInt, mut beg: i32, fin: i32) {
        if fin >= beg {
            let end = fin - beg;
            for i in 0..=end {
                tar.val[i as usize] = rhs.val[beg as usize];
                beg += 1;
            }
        }
    }

    fn get_val(&self) -> Ulli {
        let mut val: Ulli = 0;
        if self.end <= 20 {
            for i in 0..=self.end {
                // same double arithmetic as the C++ `val += digit * pow(10, i)`
                val = (val as f64 + self.val[i as usize] as f64 * 10f64.powi(i)) as Ulli;
            }
        }
        val
    }

    // operator- (note: like the C++ version it borrows from `self`'s digits, so it changes self)
    fn sub(&mut self, rhs: &BigInt) -> BigInt {
        let mut n_v = BigInt::new();
        if self.end < rhs.end {
            return BigInt::from(0);
        }
        for i in 0..=self.end {
            let iu = i as usize;
            if self.val[iu] < rhs.val[iu] {
                if i + 1 > self.end {
                    return BigInt::from(0);
                }
                self.val[iu] += 10;
                self.val[iu + 1] -= 1;
            }
            n_v.val[iu] = self.val[iu] - rhs.val[iu];
        }
        let mut e = self.end;
        while e >= 0 {
            if n_v.val[e as usize] != 0 {
                n_v.end = e;
                break;
            }
            e -= 1;
        }
        n_v
    }
}

fn main() {
    let mut b1 = BigInt::from(9);
    let mut b2 = BigInt::from(3);
    println!("{} + {} = {}", b1, b2, b1.add(&b2));
    print!("{} - {} = ", b1, b2); // printed first because sub() changes b1
    println!("{}", b1.sub(&b2));
    println!("{} * {} = {}", b1, b2, b1.mul(&b2));
    println!("{}", b1.get_val());
    b1.set(Ulli::MAX);
    println!("{}", b1);
    println!("{} + {} = {}", b1, b2, b1.add(&b2));
    println!("{} * {} = {}", b1, b2, b1.mul(&b2));
    b2.set(15);
    b2.double_me();
    println!("{}", b2);
}
