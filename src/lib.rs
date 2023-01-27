#[cfg(feature = "visualizer")]
extern crate kiss3d;

mod arrangement;
mod color;
mod light_strip;
mod loc;
mod math;
mod ntree;

pub fn add(left: usize, right: usize) -> usize {
    return left + right;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
