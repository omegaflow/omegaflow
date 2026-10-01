pub fn solve_normal_equations(
    ata: &[Vec<f64>],
    atx: &[f64],
    aty: &[f64],
    atz: &[f64],
) -> Option<(Vec<f64>, Vec<f64>, Vec<f64>)> {
    let n = ata.len();
    let mut a = ata.to_vec();
    let mut bx = atx.to_vec();
    let mut by = aty.to_vec();
    let mut bz = atz.to_vec();
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
        bx.swap(i, pivot);
        by.swap(i, pivot);
        bz.swap(i, pivot);
        for j in i + 1..n {
            let factor = a[j][i] / a[i][i];
            for k in i..n {
                let aik = a[i][k];
                a[j][k] -= factor * aik;
            }
            bx[j] -= factor * bx[i];
            by[j] -= factor * by[i];
            bz[j] -= factor * bz[i];
        }
    }
    let x = back_substitute(&a, &bx);
    let y = back_substitute(&a, &by);
    let z = back_substitute(&a, &bz);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_equations_reduce_the_rhs_with_the_matrix() {
        let ata = vec![vec![2.0, 1.0], vec![1.0, 3.0]];
        let (x, y, z) =
            solve_normal_equations(&ata, &[3.0, 5.0], &[1.0, 1.0], &[0.0, 0.0]).expect("solves");
        assert!((x[0] - 0.8).abs() < 1e-12);
        assert!((x[1] - 1.4).abs() < 1e-12);
        assert!((y[0] - 0.4).abs() < 1e-12);
        assert!((y[1] - 0.2).abs() < 1e-12);
        assert_eq!(z, vec![0.0, 0.0]);
        assert!(
            solve_normal_equations(
                &vec![vec![0.0, 0.0], vec![0.0, 1.0]],
                &[1.0, 1.0],
                &[0.0, 0.0],
                &[0.0, 0.0]
            )
            .is_none()
        );
    }
}
