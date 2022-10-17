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
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Op1::Complement, Op1::Complement) => true,
            (Op1::Degrade { new_depth: _ }, Op1::Degrade { new_depth: _ }) => true,
            (Op1::Extend, Op1::Extend) => true,
            (Op1::Contract, Op1::Contract) => true,
            (Op1::ExtBorder, Op1::ExtBorder) => true,
            (Op1::IntBorder, Op1::IntBorder) => true,
            _ => false,
        }
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
    pub fn perform_op_on_tmoc(self, moc: &Tmoc) -> Result<Tmoc, String> {
        match self {
            Op1::Complement => Ok(moc.not()),
            Op1::Degrade { new_depth } => Ok(moc.degraded(new_depth)),
            Op1::Extend => Err(String::from(
                "Extend border not implemented (yet) for T-MOCs.",
            )),
            Op1::Contract => Err(String::from(
                "Contract border not implemented (yet) for T-MOCs.",
            )),
            Op1::ExtBorder => Err(String::from(
                "External border not implemented (yet) for T-MOCs.",
            )),
            Op1::IntBorder => Err(String::from(
                "Internal border not implemented (yet) for T-MOCs.",
            )),
        }
    }
    pub fn perform_op_on_stmoc(self, _moc: &Stmoc) -> Result<Stmoc, String> {
        match self {
            Op1::Complement => Err(String::from(
                "Complement not implemented (yet) for ST-MOCs.",
            )),
            Op1::Degrade { new_depth: _ } => {
                Err(String::from("Degrade not implemented (yet) for ST-MOCs."))
            }
            Op1::Extend => Err(String::from(
                "Extend border not implemented (yet) for ST-MOCs.",
            )),
            Op1::Contract => Err(String::from(
                "Contract border not implemented (yet) for ST-MOCs.",
            )),
            Op1::ExtBorder => Err(String::from(
                "External border not implemented (yet) for ST-MOCs.",
            )),
            Op1::IntBorder => Err(String::from(
                "Internal border not implemented (yet) for ST-MOCs.",
            )),
        }
    }
}
