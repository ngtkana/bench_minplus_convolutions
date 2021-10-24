use {
    super::*,
    rand::{prelude::StdRng, Rng, SeedableRng},
};

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
