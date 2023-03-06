use core::fmt;

use crate::utils::namestore::add;
use moc::storage::u64idx::{common::MocQType, U64MocStore};

// The OP1 type
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
    // #Definition
    //      fmt formats an OP1 to a string
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
    // #Definition
    //      Checks if 2 OP1s are equal or not
    // #Args
    //  *   `other`: the other OP1 to that needs to be compared with self
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
    // #Definition
    //      perform_op_on_smoc, it does exactly as it says, it performs an operation on a given SMOC
    // #Args
    //  *   `id`: the MOC on which to perform the op
    //  *   `n`: the name given to the new MOC once added to the store
    // #Errors
    //      Errors can come if "add" does not work, in which case the MOC is not added to the store.
    fn perform_op_on_smoc(self, id: usize, n: &str) -> Result<(), String> {
        let name = n.to_string();
        match self {
            Op1::Complement => {
                if let Ok(index) = U64MocStore.complement(id) {
                    add(&name, index)?;
                }
                Ok(())
            }
            Op1::Degrade { new_depth } => {
                if let Ok(index) = U64MocStore.degrade(id, new_depth) {
                    add(&name, index)?;
                }
                Ok(())
            }
            Op1::Extend => {
                if let Ok(index) = U64MocStore.extend(id) {
                    add(&name, index)?;
                }
                Ok(())
            }
            Op1::Contract => {
                if let Ok(index) = U64MocStore.contract(id) {
                    add(&name, index)?;
                }
                Ok(())
            }
            Op1::ExtBorder => {
                if let Ok(index) = U64MocStore.ext_border(id) {
                    add(&name, index)?;
                }
                Ok(())
            }
            Op1::IntBorder => {
                if let Ok(index) = U64MocStore.int_border(id) {
                    add(&name, index)?;
                }
                Ok(())
            }
            Op1::Split => {
                for i in U64MocStore.split(id)? {
                    add(&format!("{}_{}", i, name), i)?;
                }
                Ok(())
            }
            Op1::SplitIndirect => {
                for i in U64MocStore.split(id)? {
                    add(&format!("{}_{}", i, name), i)?;
                }
                Ok(())
            }
        }
    }
    // #Definition
    //      Same as perform_op_on_smoc, but for TMOCs
    //      The only difference is that only complement and degrade work for this type of MOC
    fn perform_op_on_tmoc(self, id: usize, n: &str) -> Result<(), String> {
        let name = n.to_string();
        match self {
            Op1::Complement => {
                if let Ok(index) = U64MocStore.complement(id) {
                    add(&name, index)?;
                }
                Ok(())
            }
            Op1::Degrade { new_depth } => {
                if let Ok(index) = U64MocStore.degrade(id, new_depth) {
                    add(&name, index)?;
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

// #Definition
//      op1 performs the given operation on the given MOC and store the resulting MOC in the store.
// #Args
//  *   `id`: the MOC's id in the store
//  *   `op`: the operation that needs to be applied to the MOC
//  *   `res_name`: The name given to the result
// #Errors
//      Error if a timespace string is used as TimeSpace MOCs cannot be operated on alone.
pub(crate) fn op1(id: usize, op: Op1, res_name: &str) -> Result<(), String> {
    let moc = U64MocStore.get_qty_type(id)?;
    match moc {
        MocQType::Space => op.perform_op_on_smoc(id, res_name),
        MocQType::Time => op.perform_op_on_tmoc(id, res_name),
        MocQType::TimeSpace => Err(String::from("Operations are not implemented for ST-MOCs.")),
        MocQType::Frequency => unreachable!(),
    }
}
