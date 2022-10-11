use crate::commons::*;
use core::fmt;

#[derive(Clone)]
pub enum Op2 {
    Intersection,
    Union,
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
        }
    }
}
