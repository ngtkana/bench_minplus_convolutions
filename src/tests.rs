use {
    super::*,
    rand::{prelude::StdRng, Rng, SeedableRng},
    std::iter::repeat_with,
};

const DIFF2_MAX: i32 = 3;

fn generate_convex_vec(rng: &mut StdRng, n: usize) -> Vec<i32> {
    let a = repeat_with(|| rng.gen_range(0..=DIFF2_MAX))
        .take(n)
        .collect_vec();
    let mut a = a
        .into_iter()
        .scan(0, |state, x| {
            *state += x;
            Some(*state)
        })
        .collect_vec();
    let init = rng.gen_range(a.iter().copied().min().unwrap()..=a.iter().copied().max().unwrap());
    a.iter_mut().for_each(|x| *x -= init);
    let mut a = a
        .into_iter()
        .scan(0, |state, x| {
            *state += x;
            Some(*state)
        })
        .collect_vec();
    let init = rng.gen_range(a.iter().copied().min().unwrap()..=a.iter().copied().max().unwrap());
    a.iter_mut().for_each(|x| *x -= init);
    a
}

#[test]
fn test_convex_minplus_convolution() {
    let mut rng = StdRng::seed_from_u64(42);
    for _ in 0..2000 {
        let l = rng.gen_range(1..30);
        let m = rng.gen_range(1..30);
        let a = generate_convex_vec(&mut rng, l);
        let b = generate_convex_vec(&mut rng, m);
        let expected = brute_minplus_convolution(&a, &b);
        let monotone_minima_result = monotone_minima_minplus_convolution(&a, &b);
        let smawk_result = smawk_minplus_convolution(&a, &b);
        assert_eq!(&expected, &monotone_minima_result);
        assert_eq!(&expected, &smawk_result);
    }
}
