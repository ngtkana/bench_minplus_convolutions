#[cfg(test)]
mod tests;
use {itertools::Itertools, std::cmp::Ordering};

pub fn brute_minplus_convolution(a: &[i32], b: &[i32]) -> Vec<i32> {
    if a.is_empty() && b.is_empty() {
        return Vec::new();
    }
    let mut c = vec![i32::MAX; a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        for (j, &y) in b.iter().enumerate() {
            c[i + j] = c[i + j].min(x + y);
        }
    }
    c
}

pub fn monotone_minima_minplus_convolution(a: &[i32], b: &[i32]) -> Vec<i32> {
    if a.is_empty() && b.is_empty() {
        Vec::new()
    } else {
        let f = |i: usize, j: usize| i.checked_sub(j).and_then(|ij| b.get(ij)).map(|&y| a[j] + y);
        monotone_minima_argmin(a.len() + b.len() - 1, a.len(), |i, j, k| {
            match [f(i, j), f(i, k)] {
                [None, None] => Ordering::Equal,
                [None, Some(_)] => Ordering::Greater,
                [Some(_), None] => Ordering::Less,
                [Some(x), Some(y)] => x.cmp(&y),
            }
        })
        .into_iter()
        .enumerate()
        .map(|(i, j)| a[j] + b[i - j])
        .collect_vec()
    }
}

pub fn smawk_minplus_convolution(a: &[i32], b: &[i32]) -> Vec<i32> {
    if a.is_empty() && b.is_empty() {
        Vec::new()
    } else {
        let f = |i: usize, j: usize| i.checked_sub(j).and_then(|ij| b.get(ij)).map(|&y| a[j] + y);
        smawk_argmin(a.len() + b.len() - 1, a.len(), |i, j, k| {
            match [f(i, j), f(i, k)] {
                [None, None] => Ordering::Equal,
                [None, Some(_)] => Ordering::Greater,
                [Some(_), None] => Ordering::Less,
                [Some(x), Some(y)] => x.cmp(&y),
            }
        })
        .into_iter()
        .enumerate()
        .map(|(i, j)| a[j] + b[i - j])
        .collect_vec()
    }
}

fn monotone_minima_argmin(
    h: usize,
    w: usize,
    cmp: impl Fn(usize, usize, usize) -> Ordering + Copy,
) -> Vec<usize> {
    assert!(0 < h);
    let mut ans = vec![0; h];
    ans[0] = (0..w).rev().min_by(|&j, &k| cmp(0, j, k)).unwrap();
    for d in (0..h.next_power_of_two().trailing_zeros() as usize)
        .rev()
        .map(|d| 1 << d)
    {
        for i in (d..h).step_by(2 * d) {
            let start = ans[i - d];
            let end = ans.get(i + d).copied().unwrap_or(w - 1);
            ans[i] = (start..=end).rev().min_by(|&j, &k| cmp(i, j, k)).unwrap();
        }
    }
    ans
}

fn smawk_argmin(
    h: usize,
    w: usize,
    cmp: impl Fn(usize, usize, usize) -> Ordering + Copy,
) -> Vec<usize> {
    // Reduce
    let mut stack = vec![((0..h).collect_vec(), (0..w).collect_vec())];
    loop {
        let (rows, cols) = &stack.last().unwrap();
        if rows.len() == 1 {
            break;
        }
        let mut swp = Vec::new();
        for &k in cols {
            while let Some(j) = swp.pop() {
                if matches! { cmp(rows[swp.len()], j, k), Ordering::Less } {
                    swp.push(j);
                    break;
                }
            }
            if rows.len() == swp.len() {
                break;
            }
            swp.push(k);
        }
        let rows = rows.iter().copied().skip(1).step_by(2).collect_vec();
        stack.push((rows, swp));
    }

    // Interpolate
    let mut argmin = Vec::new();
    while let Some((rows, cols)) = stack.pop() {
        let mut swp = vec![!0; rows.len()];
        let mut iter = cols.iter().copied().peekable();
        let mut i = 0;
        for &end in &argmin {
            swp[i] = *iter.peek().unwrap();
            while let Some(&j) = iter.peek() {
                if matches! { cmp(rows[i], swp[i], j), Ordering::Equal | Ordering::Greater } {
                    swp[i] = j;
                }
                if j == end {
                    break;
                }
                iter.next().unwrap();
            }
            i += 1;
            swp[i] = end;
            i += 1;
        }
        if rows.len() % 2 == 1 {
            swp[i] = *iter.peek().unwrap();
            for j in iter {
                if matches! { cmp(rows[i], swp[i], j), Ordering::Equal | Ordering::Greater } {
                    swp[i] = j;
                }
            }
        }
        argmin = swp;
    }
    argmin
}
