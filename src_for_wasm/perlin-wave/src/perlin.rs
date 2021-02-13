use noise::{NoiseFn, OpenSimplex};

pub fn noise_2d(x: f64, y: f64) -> f64 {
    let perlin = OpenSimplex::new();
    perlin.get([x, y])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn non_zero() {
        let x: f64 = 1.0;
        let y: f64 = 1.0;
        assert!(noise_2d(x, y) > 0.0)
    }
}
