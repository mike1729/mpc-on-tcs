use bls12_381::Scalar;

#[derive(Default)]
pub struct Commitment {}

impl Commitment {
    pub fn new(_coeff: Scalar) -> Self {
        Default::default()
    }

    pub fn product(_commitments: &[Self]) -> Self {
        Default::default()
    }

    pub fn poly_eval(_coeffs: &[Self], _x: &Scalar) -> Self {
        Default::default()
    }

    pub fn verify_val(&self, _val: &Scalar) -> bool {
        true
    }
}
