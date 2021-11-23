use bls12_381::Scalar;

#[derive(Default)]
pub struct Commitment {}

impl Commitment {
    pub fn new(_coeff: Scalar) -> Self {
        Default::default()
    }

    pub fn product(_commitments: impl Iterator<Item = Self>) -> Self {
        Default::default()
    }

    pub fn poly_eval(_coeffs: impl Iterator<Item = Self>, _x: &Scalar) -> Self {
        Default::default()
    }

    pub fn verify_val(&self, _val: &Scalar) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn product() {
        let scalars = (0..10u64)
            .map(|i| [i, i + 1, i * 2, i * i])
            .map(Scalar::from_raw);

        let commitments = scalars.clone().map(Commitment::new);
        let product = Commitment::product(commitments);

        assert!(product.verify_val(&scalars.fold(Scalar::one(), |a, b| a * b)));
    }

    #[test]
    fn poly_eval() {
        let poly = [Scalar::one(), Scalar::one().double(), Scalar::one()];
        let x = Scalar::one().double();
        let target = (x + Scalar::one()).square();
        let coeffs = poly.iter().cloned().map(Commitment::new);

        let eval = Commitment::poly_eval(coeffs, &x);

        assert!(eval.verify_val(&target));
    }
}
