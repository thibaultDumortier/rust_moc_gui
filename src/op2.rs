use crate::commons::*;
use core::fmt;

#[derive(Clone)]
pub enum Op2 {
    Intersection,
    Union,
    Difference,
    Minus,
    TFold,
    SFold,
}
impl Default for Op2 {
    fn default() -> Self {
        Op2::Intersection
    }
}
impl fmt::Display for Op2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Intersection => write!(f, "Intersection"),
            Self::Union => write!(f, "Union"),
            Self::Difference => write!(f, "Difference"),
            Self::Minus => write!(f, "Minus"),
            Self::TFold => write!(f, "TFold"),
            Self::SFold => write!(f, "SFold"),
        }
    }
}
impl PartialEq for Op2 {
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }

    fn eq(&self, _other: &Self) -> bool {
        true
    }
}
impl Op2 {
    pub fn perform_op2_on_smoc(self, left: &SMOC, right: &SMOC) -> Result<SMOC, String> {
        match self {
            Op2::Intersection => Ok(left.and(right)),
            Op2::Union => Ok(left.or(right)),
            Op2::Difference => Ok(left.xor(right)),
            Op2::Minus => Ok(left.minus(right)),
            Op2::TFold => Err(String::from(
                "TimeFold operation not available on 2 S-MOCs.",
            )),
            Op2::SFold => Err(String::from(
                "SpaceFold operation not available on 2 S-MOCs.",
            )),
        }
    }
}
