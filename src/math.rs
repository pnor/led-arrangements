pub fn distance<const N: usize>(p1: &[f64; N], p2: &[f64; N]) -> f64 {
    let mut sum: f64 = 0.0;
    for i in 0..N {
        sum += (p2[i] - p1[i]).powi(2);
    }
    return sum.sqrt();
}

mod test {
    use super::*;

    #[test]
    fn test_distance() {
        assert_eq!(distance(&[0.0, 0.0], &[1.0, 0.0]), 1.0);
        assert_eq!(distance(&[0.0, 0.0], &[0.0, 1.0]), 1.0);
        assert_eq!(distance(&[0.0, 0.0], &[-1.0, 0.0]), 1.0);
        assert_eq!(distance(&[0.0, 0.0], &[0.0, -1.0]), 1.0);

        assert_eq!(distance(&[0.0, 0.0], &[1.0, 1.0]), 1.0 * (2.0_f64).sqrt());
        assert_eq!(distance(&[0.5, 1.0], &[1.0, 1.0]), 0.5);
        assert_eq!(distance(&[1.0, 0.5], &[1.0, 1.0]), 0.5);
    }
}
