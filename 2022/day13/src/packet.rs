use itertools::{EitherOrBoth::*, FoldWhile::*, Itertools};
use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Packet {
    Int(u32),
    List(Vec<Packet>),
}

impl Packet {
    fn to_list(&self) -> Vec<Self> {
        match self {
            &Self::Int(v) => vec![Self::Int(v)],
            Self::List(list) => list.clone(),
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if let (Self::Int(u), Self::Int(v)) = (self, other) {
            return u.cmp(v);
        }
        let (left, right) = (self.to_list(), other.to_list());
        left.iter()
            .zip_longest(right.iter())
            .fold_while(Ordering::Equal, |cmp, curr| {
                if !cmp.is_eq() {
                    return Done(cmp);
                }
                match curr {
                    Left(_) => Done(Ordering::Greater),
                    Right(_) => Done(Ordering::Less),
                    Both(l, r) => Continue(l.cmp(r)),
                }
            })
            .into_inner()
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
