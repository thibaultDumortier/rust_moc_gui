use crate::commons::*;
use core::fmt;

#[derive(Clone)]
pub enum Op1 {
    Complement,
    Degrade { new_depth: u8 },
    Extend,
    Contract,
    ExtBorder,
    IntBorder,
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
            Self::Degrade { new_depth: _ } => write!(f, "Degrade"),
            Self::Extend => write!(f, "Extend"),
            Self::Contract => write!(f, "Contract"),
            Self::ExtBorder => write!(f, "ExtBorder"),
            Self::IntBorder => write!(f, "IntBorder"),
        }
    }
}
impl PartialEq for Op1 {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}
impl Op1 {
    pub fn perform_op1_on_smoc(self, moc: &Smoc) -> Result<Smoc, String> {
        match self {
            Op1::Complement => Ok(moc.not()),
            Op1::Degrade { new_depth } => Ok(moc.degraded(new_depth)),
            Op1::Extend => Ok(moc.expanded()),
            Op1::Contract => Ok(moc.contracted()),
            Op1::ExtBorder => Ok(moc.external_border()),
            Op1::IntBorder => Ok(moc.internal_border()),
        }
    }
}
