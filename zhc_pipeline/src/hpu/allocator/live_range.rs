use std::ops::Index;

use zhc_ir::{AsValId, IR, OpIdRaw, ValId, ValMap};
use zhc_langs::hpulang::HpuLang;
use zhc_utils::{SafeAs, small::SmallVec, svec};

/// A point in the execution timeline.
pub type TimePoint = OpIdRaw;

/// All the time points a value is used at.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LiveRange(SmallVec<TimePoint>);

impl LiveRange {
    /// Return the last point of use of the value.
    pub fn to(&self) -> TimePoint {
        // The uses are ordered by construction, so the max is the last one.
        if self.0.len() == 1 {
            // See [1]
            self.0[0] + 1
        } else {
            *self.0.iter().last().unwrap()
        }
    }

    /// Return the next point of use of the value after `point`
    pub fn next_use(&self, point: TimePoint) -> Option<TimePoint> {
        // The uses are ordered by construction, so the first use that is greater or equal to now is
        // the next use.
        self.0.iter().copied().find(|u| *u >= point)
    }

    /// Returns an iterator over the points of use.
    #[allow(unused)]
    pub fn iter_uses(&self) -> impl Iterator<Item = TimePoint> {
        self.0.iter().cloned()
    }

    /// Whether a value is used at a given point in time.
    pub fn is_used_at(&self, point: TimePoint) -> bool {
        self.0.iter().any(|tp| *tp == point)
    }
}

/// A map from values to their live ranges.
#[derive(Debug)]
pub struct LiveRangeMap {
    ranges: ValMap<LiveRange>,
    retiring: Vec<SmallVec<ValId>>,
}

impl LiveRangeMap {
    /// Extracts the map from a scheduled IR.
    pub fn from_scheduled_ir(ir: &IR<HpuLang>) -> Self {
        use zhc_langs::hpulang::HpuTypeSystem::*;
        let mut ranges: ValMap<LiveRange> = ir.empty_valmap();
        for (point, op) in ir.walk_ops_linear().enumerate() {
            for val in op
                .get_args_iter()
                .filter(|v| matches!(v.get_type(), CtRegister))
            {
                ranges.get_mut(&val).unwrap().0.push(point.sas());
            }
            for val in op
                .get_returns_iter()
                .filter(|v| matches!(v.get_type(), CtRegister))
            {
                ranges.insert(&val, LiveRange(svec![point.sas()]));
            }
        }

        let mut retiring = vec![SmallVec::new(); ir.n_ops().sas::<usize>() + 2];
        for (valid, live_range) in ranges.iter() {
            retiring[live_range.to().sas::<usize>()].push(valid);
        }
        LiveRangeMap { ranges, retiring }
    }

    /// Returns an iterator over the values retiring at this time point.
    pub fn retiring_iter(&self, point: TimePoint) -> impl Iterator<Item = ValId> {
        self.retiring
            .get(point.sas::<usize>())
            .into_iter()
            .flat_map(|vals| vals.iter().copied())
    }
}

impl<I: AsValId> Index<I> for LiveRangeMap {
    type Output = LiveRange;

    fn index(&self, index: I) -> &LiveRange {
        &self.ranges[index]
    }
}

// Notes:
// ======
//
// [1]: Live range minimal size. If an op returns two values, one of which is not used, we must still allocate a
// register for both of them. Indeed, there is not built-in support for unused dst in doplang, and
// preventing spurious dependencies requires an equal treatment of those unused values. That said,
// in this case, only a single usage will be discovered for this value, at its creation point. To
// prevent the allocator to trying to retire this value while it has not yet been registered in the
// register file, we must make sure that the live range of a value can never be smaller than 1.
