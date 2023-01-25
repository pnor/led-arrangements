/// Interface for talking about locations in coordinate space
/// Converts cartesian, polar, and cylindrical into float arrays

pub struct Loc<const N: usize> {
    pub coords: [f64; N],
}

impl<const N: usize> Loc<N> {
    const fn one_less() -> usize {
        N - 1
    }

    pub fn cartesian(loc: [f64; N]) -> Self {
        Loc { coords: loc }
    }

    /// Create `Loc` from polar coordinates
    /// `rho` is the radius, distance from center
    /// `angular_coords` is the angular coordinates theta, phi, ...
    /// `center` is the point the coordinates should be centered on
    ///
    /// For creating an N-dimensional point, there should be N - 1 angular coordinates provided
    /// All angular coordinates from 0..=(N -2) should be 0..pi, with the final point at N - 1 be in
    /// 0..=(2 * pi))
    ///
    pub fn polar(rho: f64, angular_coords: &Vec<f64>, center: &[f64; N]) -> Self {
        assert!(angular_coords.len() == N - 1);
        let mut coords: [f64; N] = [0.0; N];

        let mut s = 1.0;
        for i in 0..angular_coords.len() {
            coords[i] = (s * angular_coords[i].cos() * rho) + center[i];
            s *= angular_coords[i].sin();
        }
        coords[N - 1] = (s * rho) + center[N - 1];

        return Loc { coords };
    }

    pub fn cylindrical(radius: f64, theta: f64, coords: Vec<f64>) -> Self {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use std::f64::consts::PI;

    use super::*;

    fn sqrt_2_over_2() -> f64 {
        (2.0_f64.sqrt()) / 2.0
    }

    fn approx<const N: usize>(&arr1: &[f64; N], &arr2: &[f64; N]) -> bool {
        let epsilon = 0.001;
        for i in 0..N {
            if (arr1[i] - arr2[i]).abs() > epsilon {
                return false;
            }
        }
        return true;
    }

    #[test]
    fn cartesian_loc() {
        // 1D
        let leftest = Loc::cartesian([0.0]);
        assert_eq!(leftest.coords, [0.0]);
        let midleft = Loc::cartesian([0.4]);
        assert_eq!(midleft.coords, [0.4]);
        let midright = Loc::cartesian([0.6]);
        assert_eq!(midright.coords, [0.6]);
        let rightest = Loc::cartesian([1.0]);
        assert_eq!(rightest.coords, [1.0]);

        // 2D
        let mid = Loc::cartesian([0.5, 0.5]);
        assert_eq!(mid.coords, [0.5, 0.5]);
        let top = Loc::cartesian([0.5, 1.0]);
        assert_eq!(top.coords, [0.5, 1.0]);
        let bottom = Loc::cartesian([0.5, 0.0]);
        assert_eq!(bottom.coords, [0.5, 0.0]);
        let left = Loc::cartesian([0.0, 0.5]);
        assert_eq!(left.coords, [0.0, 0.5]);
        let right = Loc::cartesian([1.0, 0.5]);
        assert_eq!(right.coords, [1.0, 0.5]);
        let topleft = Loc::cartesian([-sqrt_2_over_2() + 0.5, sqrt_2_over_2() + 0.5]);
        assert_eq!(
            topleft.coords,
            [-sqrt_2_over_2() + 0.5, sqrt_2_over_2() + 0.5]
        );

        // 3D
        let mid = Loc::cartesian([0.5, 0.5, 0.5]);
        assert_eq!(mid.coords, [0.5, 0.5, 0.5]);
        let mid_front = Loc::cartesian([0.5, 1.0, 0.5]);
        assert_eq!(mid_front.coords, [0.5, 1.0, 0.5]);
        let mid_back = Loc::cartesian([0.5, 0.0, 0.5]);
        assert_eq!(mid_back.coords, [0.5, 0.0, 0.5]);
        let top = Loc::cartesian([0.5, 0.5, 1.0]);
        assert_eq!(top.coords, [0.5, 0.5, 1.0]);
        let bottom = Loc::cartesian([0.5, 0.5, 0.0]);
        assert_eq!(bottom.coords, [0.5, 0.5, 0.0]);
        let left = Loc::cartesian([0.0, 0.5, 0.5]);
        assert_eq!(left.coords, [0.0, 0.5, 0.5]);
        let right = Loc::cartesian([1.0, 0.5, 0.5]);
        assert_eq!(right.coords, [1.0, 0.5, 0.5]);
        let topleftfront = Loc::cartesian([
            -sqrt_2_over_2() + 0.5,
            sqrt_2_over_2() + 0.5,
            sqrt_2_over_2() + 0.5,
        ]);
        assert_eq!(
            topleftfront.coords,
            [
                -sqrt_2_over_2() + 0.5,
                sqrt_2_over_2() + 0.5,
                sqrt_2_over_2() + 0.5
            ]
        );
    }

    #[test]
    fn polar_loc() {
        // 2D
        let mid = Loc::polar(0.0, &vec![0.0], &[0.5, 0.5]);
        assert!(approx(&mid.coords, &[0.5, 0.5]));
        let top = Loc::polar(0.5, &vec![PI / 2.0], &[0.5, 0.5]);
        assert!(approx(&top.coords, &[0.5, 1.0]));
        let bottom = Loc::polar(0.5, &vec![(3.0 * PI) / 2.0], &[0.5, 0.5]);
        assert!(approx(&bottom.coords, &[0.5, 0.0]));
        let left = Loc::polar(0.5, &vec![PI], &[0.5, 0.5]);
        assert!(approx(&left.coords, &[0.0, 0.5]));
        let right = Loc::polar(0.5, &vec![0.0], &[0.5, 0.5]);
        assert!(approx(&right.coords, &[1.0, 0.5]));
        let topleft = Loc::polar(0.5, &vec![(3.0 * PI) / 4.0], &[0.5, 0.5]);
        assert!(approx(
            &topleft.coords,
            &[
                (-sqrt_2_over_2() / 2.0) + 0.5,
                (sqrt_2_over_2() / 2.0) + 0.5
            ]
        ));

        // 3D
        let mid = Loc::polar(0.0, &vec![0.0, 0.0], &[0.5, 0.5, 0.5]);
        assert_eq!(mid.coords, [0.5, 0.5, 0.5]);
        let mid_front = Loc::polar(0.5, &vec![PI / 2.0, 0.0], &[0.5, 0.5, 0.5]);
        assert_eq!(mid_front.coords, [0.5, 1.0, 0.5]);
        let mid_back = Loc::polar(0.5, &vec![-PI / 2.0, 0.0], &[0.5, 0.5, 0.5]);
        assert_eq!(mid_back.coords, [0.5, 0.0, 0.5]);
        let top = Loc::polar(0.5, &vec![PI / 2.0, PI / 2.0], &[0.5, 0.5, 0.5]);
        assert_eq!(top.coords, [0.5, 0.5, 1.0]);
        let bottom = Loc::polar(0.5, &vec![PI / 2.0, -PI / 2.0], &[0.5, 0.5, 0.5]);
        assert_eq!(bottom.coords, [0.5, 0.5, 0.0]);
        let left = Loc::polar(0.5, &vec![PI, PI / 2.0], &[0.5, 0.5, 0.5]);
        assert!(approx(&left.coords, &[0.0, 0.5, 0.5]));
        let right = Loc::polar(0.5, &vec![0.0, PI / 2.0], &[0.5, 0.5, 0.5]);
        assert_eq!(right.coords, [1.0, 0.5, 0.5]);
        let topleftfront = Loc::polar(
            0.5,
            &vec![(3.0 * PI) / 4.0, (3.0 * PI) / 4.0],
            &[0.5, 0.5, 0.5],
        );
        assert!(approx(
            &topleftfront.coords,
            &[(-sqrt_2_over_2() / 2.0) + 0.5, 0.25, 0.75]
        ));
    }
}
