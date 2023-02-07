use core::fmt;

use crate::namestore::add;
use moc::storage::u64idx::{common::MocQType, U64MocStore};

#[derive(Copy, Clone)]
pub(crate) enum Op1 {
    Complement,
    Degrade { new_depth: u8 },
    Extend,
    Contract,
    ExtBorder,
    IntBorder,
    Split,
    SplitIndirect,
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
            Self::Split => write!(f, "Split"),
            Self::SplitIndirect => write!(f, "SplitIndirect"),
        }
    }
}

impl PartialEq for Op1 {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Op1::Complement, Op1::Complement)
                | (Op1::Degrade { new_depth: _ }, Op1::Degrade { new_depth: _ })
                | (Op1::Extend, Op1::Extend)
                | (Op1::Contract, Op1::Contract)
                | (Op1::ExtBorder, Op1::ExtBorder)
                | (Op1::IntBorder, Op1::IntBorder)
                | (Op1::Split, Op1::Split)
                | (Op1::SplitIndirect, Op1::SplitIndirect)
        )
    }
}

impl Op1 {
    fn is_split_4neigh(&self) -> bool {
        matches!(self, Op1::Split)
    }
    fn is_split_8neigh(&self) -> bool {
        matches!(self, Op1::SplitIndirect)
    }

    fn perform_op_on_smoc(self, id: usize, n: &str) -> Result<(), String> {
        let name = n.to_string();
        match self {
            Op1::Complement => {
                if let Ok(index) = U64MocStore.complement(id) {
                    add(name, index)?;
                }
                Ok(())
            }
            Op1::Degrade { new_depth } => {
                if let Ok(index) = U64MocStore.degrade(id, new_depth) {
                    add(name, index)?;
                }
                Ok(())
            }
            Op1::Extend => {
                if let Ok(index) = U64MocStore.extend(id) {
                    add(name, index)?;
                }
                Ok(())
            }
            Op1::Contract => {
                if let Ok(index) = U64MocStore.contract(id) {
                    add(name, index)?;
                }
                Ok(())
            }
            Op1::ExtBorder => {
                if let Ok(index) = U64MocStore.ext_border(id) {
                    add(name, index)?;
                }
                Ok(())
            }
            Op1::IntBorder => {
                if let Ok(index) = U64MocStore.int_border(id) {
                    add(name, index)?;
                }
                Ok(())
            }
            Op1::Split | Op1::SplitIndirect => {
                Err(String::from("Split must be catch before this :o/."))
            }
        }
    }
    fn perform_op_on_tmoc(self, id: usize, n: &str) -> Result<(), String> {
        let name = n.to_string();
        match self {
            Op1::Complement => {
                if let Ok(index) = U64MocStore.complement(id) {
                    add(name, index)?;
                }
                Ok(())
            }
            Op1::Degrade { new_depth } => {
                if let Ok(index) = U64MocStore.degrade(id, new_depth) {
                    add(name, index)?;
                }
                Ok(())
            }
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
            Op1::Split | Op1::SplitIndirect => {
                Err(String::from("Split not implemented for T-MOCs."))
            }
        }
    }
}

/// Performs the given operation on the given MOC and store the resulting MOC in the store.
pub(crate) fn op1(id: usize, op: Op1, res_name: &str) -> Result<(), String> {
    if let Ok(moc) = U64MocStore.get_qty_type(id) {
        if op.is_split_4neigh() || op.is_split_8neigh() {
            match moc {
                MocQType::Space => {
                    U64MocStore.split_count(id)?;
                    if let Ok(indexes) = U64MocStore.split(id) {
                        for i in indexes {
                            add(format!("{}({})", res_name, i), id)?;
                        }
                    }
                    Ok(())
                }
                MocQType::Time => Err(String::from("Split not implemented for T-MOCs.")),
                MocQType::TimeSpace => Err(String::from("Split not implemented for ST-MOCs.")),
                MocQType::Frequency => Err(String::from("Frequency MOCs not supported.")),
            }
        } else {
            match moc {
                MocQType::Space => op.perform_op_on_smoc(id, res_name),
                MocQType::Time => op.perform_op_on_tmoc(id, res_name),
                MocQType::TimeSpace => {
                    Err(String::from("Operations are not implemented for ST-MOCs."))
                }
                MocQType::Frequency => Err(String::from("Frequency MOCs not supported.")),
            }
        }
    } else {
        Err(String::from("Could not get moc QTY type"))
    }
}
