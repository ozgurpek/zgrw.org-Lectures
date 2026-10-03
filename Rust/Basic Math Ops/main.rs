// Rust does not let us implement operators (`*`, `<<`) for Vec, which is a
// type from the standard library, so the C++ operator overloads become
// plain functions here.

// operator*(vector, vector): dot product
fn dot(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
    let mut sum = 0;
    let sz = v1.len().min(v2.len()); //remaining dimensions for the smaller vector will be accepted as 0 as we were in the bigger dimension world
    for i in 0..sz {
        sum += v1[i] * v2[i];
    }
    sum
}

// operator*(matrix, matrix): matrix multiplication
fn mat_mul(m1: &Vec<Vec<i32>>, m2: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let sz1 = m1.len();
    let sz2 = m2.len();
    let mut res: Vec<Vec<i32>> = Vec::new();
    if sz1 == m2[0].len() && sz2 == m1[0].len() {
        for i in 0..sz1 {
            let mut row: Vec<i32> = Vec::new();
            for k in 0..sz1 {
                let mut sum = 0;
                for j in 0..sz2 {
                    sum += m1[i][j] * m2[j][k];
                }
                row.push(sum);
            }
            res.push(row);
        }
    }
    res
}

// operator<<(ostream, matrix): one row per line
fn matrix_to_string(m1: &Vec<Vec<i32>>) -> String {
    let mut out = String::new();
    for ml in m1 {
        for ms in ml {
            out += &format!("{} ", ms);
        }
        out += "\n";
    }
    out
}

fn main() {
    let v1 = vec![1, 2, 3];
    let v2 = vec![3, 2, 1];
    println!("v1 * v2 = {}", dot(&v1, &v2));
    let m1 = vec![vec![1, 2, 3], vec![4, 5, 6]];
    let m2 = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
    println!("m1 * m2; \n{}", matrix_to_string(&mat_mul(&m1, &m2)));
}
