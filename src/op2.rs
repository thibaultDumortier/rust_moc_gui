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
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Op2::Intersection, Op2::Intersection) => true,
            (Op2::Union, Op2::Union) => true,
            (Op2::Difference, Op2::Difference) => true,
            (Op2::Minus, Op2::Minus) => true,
            (Op2::TFold, Op2::TFold) => true,
            (Op2::SFold, Op2::SFold) => true,
            _ => false,
        }
    }
}
impl Op2 {
    pub fn perform_op2_on_smoc(self, left: &Smoc, right: &Smoc) -> Result<Smoc, String> {
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
