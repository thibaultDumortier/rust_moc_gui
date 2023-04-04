use core::fmt;

use crate::utils::namestore::add;
use moc::storage::u64idx::U64MocStore;

// The OP2 type
#[derive(Copy, Clone, Eq)]
pub(crate) enum Op2 {
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
    // #Definition
    //      fmt formats an OP2 to a string
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
    // #Definition
    //      Checks if 2 OP2s are equal or not
    // #Args
    //  *   `other`: the other OP2 to that needs to be compared with self
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Op2::Intersection, Op2::Intersection)
                | (Op2::Union, Op2::Union)
                | (Op2::Difference, Op2::Difference)
                | (Op2::Minus, Op2::Minus)
                | (Op2::TFold, Op2::TFold)
                | (Op2::SFold, Op2::SFold)
        )
    }
}

impl Op2 {
    // #Definition
    //      perform_op_on_smoc, it does exactly as it says, it performs an operation on a given SMOC
    // #Args
    //  *   `left`: the first selected MOC
    //  *   `right`: the second MOC on which the op will be performed, using both MOCs
    //  *   `n`: the name given to the new MOC once added to the store
    // #Errors
    //      Errors can come if "add" does not work, in which case the MOC is not added to the store.
    fn perform_op_on_smoc(self, left: usize, right: usize, n: &str) -> Result<(), String> {
        let name = n.to_string();
        match self {
            Op2::Intersection => {
                if let Ok(index) = U64MocStore.intersection(left, right) {
                    add(&name, index)?;
                }
                Ok(())
            }
            Op2::Union => {
                if let Ok(index) = U64MocStore.union(left, right) {
                    add(&name, index)?;
                }
                Ok(())
            }
            Op2::Difference => {
                if let Ok(index) = U64MocStore.difference(left, right) {
                    add(&name, index)?;
                }
                Ok(())
            }
            Op2::Minus => {
                if let Ok(index) = U64MocStore.minus(left, right) {
                    add(&name, index)?;
                }
                Ok(())
            }
            Op2::TFold => Err(String::from(
                "TimeFold operation not available on 2 S-MOCs.",
            )),
            Op2::SFold => Err(String::from(
                "SpaceFold operation not available on 2 S-MOCs.",
            )),
        }
    }
    // #Definition
    //      Same as perform_op_on_smoc, but for TMOCs
    fn perform_op_on_tmoc(self, left: usize, right: usize, n: &str) -> Result<(), String> {
        let name = n.to_string();
        match self {
            Op2::Intersection => {
                if let Ok(index) = U64MocStore.intersection(left, right) {
                    add(&name, index)?;
                }
                Ok(())
            }
            Op2::Union => {
                if let Ok(index) = U64MocStore.union(left, right) {
                    add(&name, index)?;
                }
                Ok(())
            }
            Op2::Difference => {
                if let Ok(index) = U64MocStore.difference(left, right) {
                    add(&name, index)?;
                }
                Ok(())
            }
            Op2::Minus => {
                if let Ok(index) = U64MocStore.minus(left, right) {
                    add(&name, index)?;
                }
                Ok(())
            }
            Op2::TFold => Err(String::from(
                "TimeFold operation not available on 2 T-MOCs.",
            )),
            Op2::SFold => Err(String::from(
                "SpaceFold operation not available on 2 T-MOCs.",
            )),
        }
    }
    // #Definition
    //      Same as perform_op_on_smoc, but for STMOCs
    //      The difference is that difference does not work for STMOCs
    fn perform_op_on_stmoc(self, left: usize, right: usize, n: &str) -> Result<(), String> {
        let name = n.to_string();
        match self {
            Op2::Intersection => {
                if let Ok(index) = U64MocStore.intersection(left, right) {
                    add(&name, index)?;
                }
                Ok(())
            }
            Op2::Union => {
                if let Ok(index) = U64MocStore.union(left, right) {
                    add(&name, index)?;
                }
                Ok(())
            }
            Op2::Difference => {
                Err(String::from(
                    "Difference (or xor) not implemented for ST-MOCs.",
                ))
            }
            Op2::Minus => {
                if let Ok(index) = U64MocStore.minus(left, right) {
                    add(&name, index)?;
                }
                Ok(())
            }
            Op2::TFold => {
                Err(String::from(
                    "TimeFold operation not available on 2 ST-MOCs.",
                ))
            }
            Op2::SFold => {
                Err(String::from(
                    "SpaceFold operation not available on 2 ST-MOCs.",
                ))
            }
        }
    }
    // #Definition
    //      Same as perform_op_on_smoc, but it only works for space_fold
    fn perform_space_fold(self, left: usize, right: usize, n: &str) -> Result<(), String> {
        let name = n.to_string();
        if !matches!(self, Op2::SFold) {
            Err(String::from(
                "Operation SpaceFold expected on S-MOC with ST-MOC.",
            ))
        } else {
            if let Ok(index) = U64MocStore.space_fold(left, right) {
                add(&name, index)?;
            }
            Ok(())
        }
    }
    // #Definition
    //      Same as perform_op_on_smoc, but it only works for time_fold
    fn perform_time_fold(self, left: usize, right: usize, n: &str) -> Result<(), String> {
        let name = n.to_string();
        if !matches!(self, Op2::TFold) {
            Err(String::from(
                "Operation TimeFold expected on T-MOC with ST-MOC.",
            ))
        } else {
            if let Ok(index) = U64MocStore.time_fold(left, right) {
                add(&name, index)?;
            }
            Ok(())
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
//      Error if MOCs are not the same OR a TimeSpace and a Space/Time MOC.
//      Error if the MOC type is not found.
pub(crate) fn op2(left_id: usize, right_id: usize, op: Op2, res_name: &str) -> Result<(), String> {
    if let (Ok(left), Ok(right)) = (
        U64MocStore.get_qty_type(left_id),
        U64MocStore.get_qty_type(right_id),
    ) {
        match (left, right) {
            (
                moc::storage::u64idx::common::MocQType::Space,
                moc::storage::u64idx::common::MocQType::Space,
            ) => op.perform_op_on_smoc(left_id, right_id, res_name),
            (
                moc::storage::u64idx::common::MocQType::Space,
                moc::storage::u64idx::common::MocQType::TimeSpace,
            ) => op.perform_space_fold(left_id, right_id, res_name),
            (
                moc::storage::u64idx::common::MocQType::Time,
                moc::storage::u64idx::common::MocQType::Time,
            ) => op.perform_op_on_tmoc(left_id, right_id, res_name),
            (
                moc::storage::u64idx::common::MocQType::Time,
                moc::storage::u64idx::common::MocQType::TimeSpace,
            ) => op.perform_time_fold(left_id, right_id, res_name),
            (
                moc::storage::u64idx::common::MocQType::TimeSpace,
                moc::storage::u64idx::common::MocQType::TimeSpace,
            ) => op.perform_op_on_stmoc(left_id, right_id, res_name),
            _ => Err(String::from(
                "Both type of both MOCs must be the same, except in fold operations",
            )),
        }
    } else {
        Err(String::from("Could not get moc QTY type"))
    }
}
