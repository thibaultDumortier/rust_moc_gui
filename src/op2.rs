use core::fmt;
use crate::app::*;

pub enum Op2 {
    AND,
    OR,
}
impl Default for Op2 {
    fn default() -> Self {
        Op2::AND
    }
}
impl fmt::Display for Op2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AND => write!(f, "Intersection"),
            Self::OR => write!(f, "Union"),
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
            Op2::AND => Ok(left.and(right)),
            Op2::OR => Ok(left.or(right)),
        }
    }
}