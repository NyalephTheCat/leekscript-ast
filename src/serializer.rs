use std::ops::AddAssign;

use crate::visitor::{Visitable, VisitableMut, Visitor, VisitorMut};

#[derive(Debug, Default, Clone)]
pub struct Writer(pub String);

impl AddAssign<&str> for Writer {
    fn add_assign(&mut self, rhs: &str) {
        self.0.push_str(rhs);
    }
}

impl AddAssign<String> for Writer {
    fn add_assign(&mut self, rhs: String) {
        self.0.push_str(&rhs);
    }
}

impl Visitor for Writer {
    fn visit<T>(&mut self, t: &T)
    where
        T: Visitable<Self> + ?Sized,
    {
        t.accept(self);
    }
}
