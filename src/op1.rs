use core::fmt;

use moc::moc::{CellMOCIntoIterator, CellMOCIterator, RangeMOCIterator};

use super::commons::{InternalMoc, Smoc, Stmoc, Tmoc};
use super::store;

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

    fn perform_op_on_smoc(self, moc: &Smoc) -> Result<Smoc, String> {
        match self {
            Op1::Complement => Ok(moc.not()),
            Op1::Degrade { new_depth } => Ok(moc.degraded(new_depth)),
            Op1::Extend => Ok(moc.expanded()),
            Op1::Contract => Ok(moc.contracted()),
            Op1::ExtBorder => Ok(moc.external_border()),
            Op1::IntBorder => Ok(moc.internal_border()),
            Op1::Split | Op1::SplitIndirect => {
                Err(String::from("Split must be catch before this :o/."))
            }
        }
    }
    fn perform_op_on_tmoc(self, moc: &Tmoc) -> Result<Tmoc, String> {
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
            Op1::Split | Op1::SplitIndirect => {
                Err(String::from("Split not implemented for T-MOCs."))
            }
        }
    }
    fn perform_op_on_stmoc(self, _moc: &Stmoc) -> Result<Stmoc, String> {
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
            Op1::Split | Op1::SplitIndirect => {
                Err(String::from("Split not implemented for ST-MOCs."))
            }
        }
    }
}

pub(crate) fn op1_count_split(name: &str, indirect_neigh: bool) -> Result<u32, String> {
    store::op1_gen(name, move |moc| match moc {
        InternalMoc::Space(m) => Ok(m.split_into_joint_mocs(indirect_neigh).len() as u32),
        InternalMoc::Time(_) => Err(String::from("Split not implemented for T-MOCs.")),
        InternalMoc::TimeSpace(_) => Err(String::from("Split not implemented for ST-MOCs.")),
    })
}

/// Performs the given operation on the given MOC and store the resulting MOC in the store.
pub(crate) fn op1(name: &str, op: Op1, res_name: &str) -> Result<(), String> {
    if op.is_split_4neigh() || op.is_split_8neigh() {
        store::op1_multi_res(
            name,
            move |moc| match moc {
                InternalMoc::Space(m) => {
                    let mut cellmocs = m.split_into_joint_mocs(op.is_split_8neigh());
                    Ok(cellmocs
                        .drain(..)
                        .map(|cell_moc| {
                            InternalMoc::Space(
                                cell_moc.into_cell_moc_iter().ranges().into_range_moc(),
                            )
                        })
                        .collect())
                }
                InternalMoc::Time(_) => Err(String::from("Split not implemented for T-MOCs.")),
                InternalMoc::TimeSpace(_) => {
                    Err(String::from("Split not implemented for ST-MOCs."))
                }
            },
            res_name,
        )
    } else {
        store::op1(
            name,
            move |moc| match moc {
                InternalMoc::Space(m) => op.perform_op_on_smoc(m).map(InternalMoc::Space),
                InternalMoc::Time(m) => op.perform_op_on_tmoc(m).map(InternalMoc::Time),
                InternalMoc::TimeSpace(m) => op.perform_op_on_stmoc(m).map(InternalMoc::TimeSpace),
            },
            res_name,
        )
    }
}
