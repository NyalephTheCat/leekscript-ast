use std::marker::PhantomData;

pub mod writer;

/// The Visitor trait with a generic visit method.
pub trait Visitor {
    fn visit<T>(&mut self, t: &T)
    where
        T: Visitable<Self> + ?Sized,
    {
        t.accept(self);
    }
}

/// The VisitorMut trait with a generic visit method for mutable references.
pub trait VisitorMut: Clone + Copy {
    fn visit_mut<T>(&mut self, t: &mut T)
    where
        T: VisitableMut<Self> + ?Sized,
    {
        t.accept_mut(self);
    }
}

/// The Visitable trait that types implement to accept a visitor.
pub trait Visitable<V: Visitor + ?Sized> {
    fn accept(&self, visitor: &mut V);
}

/// The VisitableMut trait for mutable visitors.
pub trait VisitableMut<V: VisitorMut + ?Sized> {
    fn accept_mut(&mut self, visitor: &mut V);
}

impl<V, T> Visitable<V> for Option<T>
where
    V: Visitor,
    T: Visitable<V>,
{
    default fn accept(&self, visitor: &mut V) {
        if let Some(value) = self {
            visitor.visit(value);
        }
    }
}

impl<V, T> VisitableMut<V> for Option<T>
where
    V: VisitorMut,
    T: VisitableMut<V>,
{
    default fn accept_mut(&mut self, visitor: &mut V) {
        if let Some(value) = self {
            visitor.visit_mut(value);
        }
    }
}

impl<V, T> Visitable<V> for Vec<T>
where
    V: Visitor,
    T: Visitable<V>,
{
    default fn accept(&self, visitor: &mut V) {
        for item in self {
            visitor.visit(item);
        }
    }
}

impl<V, T> VisitableMut<V> for Vec<T>
where
    V: VisitorMut,
    T: VisitableMut<V>,
{
    default fn accept_mut(&mut self, visitor: &mut V) {
        for item in self {
            visitor.visit_mut(item);
        }
    }
}

// Default implementation for tuples
impl<V, T1, T2> Visitable<V> for (T1, T2)
where
    V: Visitor,
    T1: Visitable<V>,
    T2: Visitable<V>,
{
    default fn accept(&self, visitor: &mut V) {
        visitor.visit(&self.0);
        visitor.visit(&self.1);
    }
}

impl<V, T1, T2> VisitableMut<V> for (T1, T2)
where
    V: VisitorMut,
    T1: VisitableMut<V>,
    T2: VisitableMut<V>,
{
    default fn accept_mut(&mut self, visitor: &mut V) {
        visitor.visit_mut(&mut self.0);
        visitor.visit_mut(&mut self.1);
    }
}

impl<V, T1, T2, T3> Visitable<V> for (T1, T2, T3)
where
    V: Visitor,
    T1: Visitable<V>,
    T2: Visitable<V>,
    T3: Visitable<V>,
{
    default fn accept(&self, visitor: &mut V) {
        visitor.visit(&self.0);
        visitor.visit(&self.1);
        visitor.visit(&self.2);
    }
}

impl<V, T1, T2, T3> VisitableMut<V> for (T1, T2, T3)
where
    V: VisitorMut,
    T1: VisitableMut<V>,
    T2: VisitableMut<V>,
    T3: VisitableMut<V>,
{
    default fn accept_mut(&mut self, visitor: &mut V) {
        visitor.visit_mut(&mut self.0);
        visitor.visit_mut(&mut self.1);
        visitor.visit_mut(&mut self.2);
    }
}

impl<V, T1, T2, T3, T4> Visitable<V> for (T1, T2, T3, T4)
where
    V: Visitor,
    T1: Visitable<V>,
    T2: Visitable<V>,
    T3: Visitable<V>,
    T4: Visitable<V>,
{
    default fn accept(&self, visitor: &mut V) {
        visitor.visit(&self.0);
        visitor.visit(&self.1);
        visitor.visit(&self.2);
        visitor.visit(&self.3);
    }
}

impl<V, T1, T2, T3, T4> VisitableMut<V> for (T1, T2, T3, T4)
where
    V: VisitorMut,
    T1: VisitableMut<V>,
    T2: VisitableMut<V>,
    T3: VisitableMut<V>,
    T4: VisitableMut<V>,
{
    default fn accept_mut(&mut self, visitor: &mut V) {
        visitor.visit_mut(&mut self.0);
        visitor.visit_mut(&mut self.1);
        visitor.visit_mut(&mut self.2);
        visitor.visit_mut(&mut self.3);
    }
}

impl<V, T> Visitable<V> for PhantomData<T>
where
    V: Visitor,
{
    default fn accept(&self, _visitor: &mut V) {}
}

impl<V, T> VisitableMut<V> for PhantomData<T>
where
    V: VisitorMut,
{
    default fn accept_mut(&mut self, _visitor: &mut V) {}
}
