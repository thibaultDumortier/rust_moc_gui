use core::fmt;

use crate::namestore::add;
use moc::storage::u64idx::U64MocStore;

#[derive(Copy, Clone)]
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
    fn perform_op_on_smoc(self, left: usize, right: usize, n: &str) -> Result<(), String> {
        let name = n.to_string();
        match self {
            Op2::Intersection => {
                if let Ok(index) = U64MocStore.intersection(left, right) {
                    add(name, index);
                }
                Ok(())
            }
            Op2::Union => {
                if let Ok(index) = U64MocStore.union(left, right) {
                    add(name, index);
                }
                Ok(())
            }
            Op2::Difference => {
                if let Ok(index) = U64MocStore.difference(left, right) {
                    add(name, index);
                }
                Ok(())
            }
            Op2::Minus => {
                if let Ok(index) = U64MocStore.minus(left, right) {
                    add(name, index);
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

    fn perform_op_on_tmoc(self, left: usize, right: usize, n: &str) -> Result<(), String> {
        let name = n.to_string();
        match self {
            Op2::Intersection => {
                if let Ok(index) = U64MocStore.intersection(left, right) {
                    add(name, index);
                }
                Ok(())
            }
            Op2::Union => {
                if let Ok(index) = U64MocStore.union(left, right) {
                    add(name, index);
                }
                Ok(())
            }
            Op2::Difference => {
                if let Ok(index) = U64MocStore.difference(left, right) {
                    add(name, index);
                }
                Ok(())
            }
            Op2::Minus => {
                if let Ok(index) = U64MocStore.minus(left, right) {
                    add(name, index);
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

    fn perform_op_on_stmoc(self, left: usize, right: usize, n: &str) -> Result<(), String> {
        let name = n.to_string();
        match self {
            Op2::Intersection => {
                if let Ok(index) = U64MocStore.intersection(left, right) {
                    add(name, index);
                }
                Ok(())
            }
            Op2::Union => {
                if let Ok(index) = U64MocStore.union(left, right) {
                    add(name, index);
                }
                Ok(())
            }
            Op2::Difference => {
                return Err(String::from(
                    "Difference (or xor) not implemented yet for ST-MOCs.",
                ))
            }
            Op2::Minus => {
                if let Ok(index) = U64MocStore.minus(left, right) {
                    add(name, index);
                }
                Ok(())
            }
            Op2::TFold => {
                return Err(String::from(
                    "TimeFold operation not available on 2 ST-MOCs.",
                ))
            }
            Op2::SFold => {
                return Err(String::from(
                    "SpaceFold operation not available on 2 ST-MOCs.",
                ))
            }
        }
    }

    fn perform_space_fold(self, left: usize, right: usize, n: &str) -> Result<(), String> {
        let name = n.to_string();
        if !matches!(self, Op2::SFold) {
            Err(String::from(
                "Operation SpaceFold expected on S-MOC with ST-MOC.",
            ))
        } else {
            if let Ok(index) = U64MocStore.space_fold(left, right) {
                add(name, index);
            }
            Ok(())
        }
    }

    fn perform_time_fold(self, left: usize, right: usize, n: &str) -> Result<(), String> {
        let name = n.to_string();
        if !matches!(self, Op2::TFold) {
            Err(String::from(
                "Operation TimeFold expected on T-MOC with ST-MOC.",
            ))
        } else {
            if let Ok(index) = U64MocStore.time_fold(left, right) {
                add(name, index);
            }
            Ok(())
        }
    }
}

/// Performs the given operation on the given MOCs and store the resulting MOC in the store.
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
