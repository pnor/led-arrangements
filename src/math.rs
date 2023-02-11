pub fn distance<const N: usize>(p1: &[f64; N], p2: &[f64; N]) -> f64 {
    let mut sum: f64 = 0.0;
    for i in 0..N {
        sum += (p2[i] - p1[i]).powi(2);
    }
    return sum.sqrt();
}

pub fn array_map<const N: usize>(arr: &[f64; N], func: &dyn Fn(&f64) -> f64) -> [f64; N] {
    arr.iter()
        .map(func)
        .collect::<Vec<f64>>()
        .try_into()
        .expect(
            "Dimension of point did not match generic
        parameter N",
        )
}

pub fn array_zip<const N: usize>(
    arr1: &[f64; N],
    arr2: &[f64; N],
    func: &dyn Fn((&f64, &f64)) -> f64,
) -> [f64; N] {
    arr1.iter()
        .zip(arr2.iter())
        .map(func)
        .collect::<Vec<f64>>()
        .try_into()
        .expect(
            "Dimension of point did not match generic
        parameter N",
        )
}

#[cfg(test)]
mod test {
    use super::*;

    pub fn clamp(number: f64, lower: f64, upper: f64) -> f64 {
        if number < lower {
            return lower;
        } else if number > upper {
            return upper;
        } else {
            return number;
        }
    }

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

    #[test]
    fn test_array_map() {
        let arr = [0.0, 1.0, 2.0, 3.0];
        let res = array_map(&arr, &|x| x + 0.5);

        assert_eq!(arr, [0.0, 1.0, 2.0, 3.0]);
        assert_eq!(res, [0.5, 1.5, 2.5, 3.5]);

        let res2 = array_map(&arr, &|x| x * 0.5);
        assert_eq!(res2, [0.0, 0.5, 1.0, 1.5]);
    }

    #[test]
    fn test_array_zip() {
        let arr1 = [0.0, 1.0, 2.0, 3.0];
        let arr2 = [3.0, 2.0, 1.0, 0.0];
        let res = array_zip(&arr1, &arr2, &|(x, y)| x * y);

        assert_eq!(arr1, [0.0, 1.0, 2.0, 3.0]);
        assert_eq!(arr2, [3.0, 2.0, 1.0, 0.0]);
        assert_eq!(res, [0.0, 2.0, 2.0, 0.0]);
    }

    #[test]
    fn test_clamp() {
        assert_eq!(clamp(1.0, 0.0, 2.0), 1.0);
        assert_eq!(clamp(0.0, 0.0, 2.0), 0.0);
        assert_eq!(clamp(-1.0, 0.0, 2.0), 0.0);
        assert_eq!(clamp(3.0, 0.0, 2.0), 2.0);
    }
}
