pub fn solve_normal_equations(
    ata: &[Vec<f64>],
    atx: &[f64],
    aty: &[f64],
    atz: &[f64],
) -> Option<(Vec<f64>, Vec<f64>, Vec<f64>)> {
    let n = ata.len();
    let mut a = ata.to_vec();
    for i in 0..n {
        let mut pivot = i;
        for j in i + 1..n {
            if a[j][i].abs() > a[pivot][i].abs() {
                pivot = j;
            }
        }
        if a[pivot][i].abs() < 1e-15 {
            return None;
        }
        a.swap(i, pivot);
        for j in i + 1..n {
            let factor = a[j][i] / a[i][i];
            let (head, tail) = a.split_at_mut(j);
            let ai = &head[i];
            let aj = &mut tail[0];
            for (ajk, aik) in aj[i..].iter_mut().zip(&ai[i..]) {
                *ajk -= factor * aik;
            }
        }
    }
    let x = back_substitute(&a, atx);
    let y = back_substitute(&a, aty);
    let z = back_substitute(&a, atz);
    Some((x, y, z))
}

pub fn back_substitute(a: &[Vec<f64>], b: &[f64]) -> Vec<f64> {
    let n = a.len();
    let mut x = b.to_vec();
    for i in (0..n).rev() {
        for j in i + 1..n {
            x[i] -= a[i][j] * x[j];
        }
        x[i] /= a[i][i];
    }
    x
}
