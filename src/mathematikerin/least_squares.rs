pub fn solve_normal_equations(
    ata: &[Vec<f64>],
    atx: &[f64],
    aty: &[f64],
    atz: &[f64],
) -> Option<(Vec<f64>, Vec<f64>, Vec<f64>)> {
    solve_normal_equations_with_pivot_ratio(ata, atx, aty, atz).map(|(x, y, z, _)| (x, y, z))
}

pub fn solve_normal_equations_with_pivot_ratio(
    ata: &[Vec<f64>],
    atx: &[f64],
    aty: &[f64],
    atz: &[f64],
) -> Option<(Vec<f64>, Vec<f64>, Vec<f64>, f64)> {
    let n = ata.len();
    let mut a = ata.to_vec();
    let mut bx = atx.to_vec();
    let mut by = aty.to_vec();
    let mut bz = atz.to_vec();
    let mut min_pivot = f64::INFINITY;
    let mut max_pivot = 0.0f64;
    for i in 0..n {
        let mut pivot = i;
        for j in i + 1..n {
            if a[j][i].abs() > a[pivot][i].abs() {
                pivot = j;
            }
        }
        let magnitude = a[pivot][i].abs();
        if magnitude < 1e-15 {
            return None;
        }
        if magnitude < min_pivot {
            min_pivot = magnitude;
        }
        if magnitude > max_pivot {
            max_pivot = magnitude;
        }
        a.swap(i, pivot);
        bx.swap(i, pivot);
        by.swap(i, pivot);
        bz.swap(i, pivot);
        for j in i + 1..n {
            let factor = a[j][i] / a[i][i];
            let (top, bottom) = a.split_at_mut(j);
            let row_i = &top[i];
            let row_j = &mut bottom[0];
            for (aik, ajk) in row_i[i..n].iter().zip(row_j[i..n].iter_mut()) {
                *ajk -= factor * aik;
            }
            bx[j] -= factor * bx[i];
            by[j] -= factor * by[i];
            bz[j] -= factor * bz[i];
        }
    }
    let x = back_substitute(&a, &bx);
    let y = back_substitute(&a, &by);
    let z = back_substitute(&a, &bz);
    let pivot_ratio = if min_pivot.is_finite() {
        max_pivot / min_pivot
    } else {
        1.0
    };
    Some((x, y, z, pivot_ratio))
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
                &[vec![0.0, 0.0], vec![0.0, 1.0]],
                &[1.0, 1.0],
                &[0.0, 0.0],
                &[0.0, 0.0]
            )
            .is_none()
        );
    }

    #[test]
    fn pivot_ratio_reads_the_elimination_pivots() {
        let identity = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
        let (_, _, _, ratio) = solve_normal_equations_with_pivot_ratio(
            &identity,
            &[1.0, 1.0],
            &[0.0, 0.0],
            &[0.0, 0.0],
        )
        .expect("the identity solves");
        assert!(
            (ratio - 1.0).abs() < 1e-12,
            "an identity carries pivot ratio 1"
        );

        let stretched = vec![vec![1.0, 0.0], vec![0.0, 1000.0]];
        let (_, _, _, ratio) = solve_normal_equations_with_pivot_ratio(
            &stretched,
            &[1.0, 1.0],
            &[0.0, 0.0],
            &[0.0, 0.0],
        )
        .expect("the stretched diagonal solves");
        assert!(
            (ratio - 1000.0).abs() < 1e-9,
            "the pivot ratio is max|pivot| / min|pivot|, not a 2-norm condition"
        );

        let near_singular = vec![vec![1.0, 0.0], vec![0.0, 1e-10]];
        let (_, _, _, ratio) = solve_normal_equations_with_pivot_ratio(
            &near_singular,
            &[1.0, 1.0],
            &[0.0, 0.0],
            &[0.0, 0.0],
        )
        .expect("the near-singular diagonal still solves above the floor");
        assert!(
            (ratio - 1e10).abs() / 1e10 < 1e-6,
            "a near-singular witness system names its degradation"
        );
    }
}
