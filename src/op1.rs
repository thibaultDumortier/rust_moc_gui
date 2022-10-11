use crate::commons::*;
use core::fmt;

#[derive(Clone)]
pub enum Op1 {
    Complement
}
impl Default for Op1 {
    fn default() -> Self {
        Op1::Complement
    }
}
impl fmt::Display for Op1 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Complement => write!(f, "Complement"),
        }
    }
}
impl PartialEq for Op1 {
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }

    fn eq(&self, _other: &Self) -> bool {
        true
    }
}
impl Op1 {
    pub fn perform_op1_on_smoc(self, moc: &SMOC) -> Result<SMOC, String> {
        match self {
            Op1::Complement => Ok(moc.not()),
        }
    }
}