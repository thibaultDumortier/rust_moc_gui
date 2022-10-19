use moc::{
    elemset::range::MocRanges,
    hpxranges2d::TimeSpaceMoc,
    moc::range::RangeMOC,
    moc2d::{range::RangeMOC2, HasTwoMaxDepth, RangeMOC2IntoIterator},
    qty::{Hpx, Time},
};

use crate::commons::*;
use core::fmt;

#[derive(Clone)]
pub enum Op2 {
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
    pub fn perform_op2_on_smoc(self, left: &Smoc, right: &Smoc) -> Result<Smoc, String> {
        match self {
            Op2::Intersection => Ok(left.and(right)),
            Op2::Union => Ok(left.or(right)),
            Op2::Difference => Ok(left.xor(right)),
            Op2::Minus => Ok(left.minus(right)),
            Op2::TFold => Err(String::from(
                "TimeFold operation not available on 2 S-MOCs.",
            )),
            Op2::SFold => Err(String::from(
                "SpaceFold operation not available on 2 S-MOCs.",
            )),
        }
    }

    pub fn perform_op2_on_tmoc(self, left: &Tmoc, right: &Tmoc) -> Result<Tmoc, String> {
        match self {
            Op2::Intersection => Ok(left.and(right)),
            Op2::Union => Ok(left.or(right)),
            Op2::Difference => Ok(left.xor(right)),
            Op2::Minus => Ok(left.minus(right)),
            Op2::TFold => Err(String::from(
                "TimeFold operation not available on 2 T-MOCs.",
            )),
            Op2::SFold => Err(String::from(
                "SpaceFold operation not available on 2 T-MOCs.",
            )),
        }
    }

    pub fn perform_op2_on_stmoc(self, left: &Stmoc, right: &Stmoc) -> Result<Stmoc, String> {
        let (time_depth_l, hpx_depth_l) = (left.depth_max_1(), left.depth_max_2());
        let (time_depth_r, hpx_depth_r) = (right.depth_max_1(), right.depth_max_2());
        // Here we loose time by performing a conversion!! (TODO implement operations on RangeMOC2!)
        let left = TimeSpaceMoc::from_ranges_it_gen(left.into_range_moc2_iter());
        let right = TimeSpaceMoc::from_ranges_it_gen(right.into_range_moc2_iter());
        let result = match self {
            Op2::Intersection => left.intersection(&right),
            Op2::Union => left.union(&right),
            Op2::Difference => {
                return Err(String::from(
                    "Difference (or xor) not implemented yet for ST-MOCs.",
                ))
            }
            Op2::Minus => left.difference(&right),
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
        };
        let time_depth = time_depth_l.max(time_depth_r);
        let space_depth = hpx_depth_l.max(hpx_depth_r);
        Ok(RangeMOC2::new(
            time_depth,
            space_depth,
            result.time_space_iter(time_depth, space_depth).collect(),
        ))
    }

    pub fn perform_space_fold(self, left: &Smoc, right: &Stmoc) -> Result<Tmoc, String> {
        if !matches!(self, Op2::SFold) {
            Err(String::from(
                "Operation SpaceFold expected on S-MOC vs ST-MOC.",
            ))
        } else {
            let time_depth = right.depth_max_1();
            // Here we loose time by performing a conversion!! (TODO implement operations on RangeMOC2!)
            let stmoc = TimeSpaceMoc::from_ranges_it_gen(right.into_range_moc2_iter());
            let tranges: MocRanges<u64, Time<u64>> =
                TimeSpaceMoc::project_on_first_dim(left.moc_ranges(), &stmoc);
            Ok(RangeMOC::new(time_depth, tranges))
        }
    }

    pub fn perform_time_fold(self, left: &Tmoc, right: &Stmoc) -> Result<Smoc, String> {
        if !matches!(self, Op2::TFold) {
            Err(String::from(
                "Operation TimeFold expected on T-MOC vs ST-MOC.",
            ))
        } else {
            let hpx_depth = right.depth_max_2();
            // Here we loose time by performing a conversion!! (TODO implement operations on RangeMOC2!)
            let stmoc = TimeSpaceMoc::from_ranges_it_gen(right.into_range_moc2_iter());
            let sranges: MocRanges<u64, Hpx<u64>> =
                TimeSpaceMoc::project_on_second_dim(left.moc_ranges(), &stmoc);
            Ok(RangeMOC::new(hpx_depth, sranges))
        }
    }
}
