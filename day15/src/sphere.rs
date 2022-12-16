use std::ops::RangeInclusive;

use crate::pos::Pos;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Sphere {
    pub center: Pos,
    pub radius: i64,
}

impl Sphere {
    pub fn bounds(&self) -> [Pos; 2] {
        [self.center - self.radius, self.center + self.radius]
    }

    pub fn raycast_y(&self, y: &i64) -> Option<RangeInclusive<i64>> {
        let [min, max] = self.bounds();
        if !(min.y..=max.y).contains(y) {
            return None;
        }
        let dist = self.radius - (self.center.y - y).abs();
        Some((self.center.x - dist)..=(self.center.x + dist))
    }
}
