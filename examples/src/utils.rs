#![allow(dead_code)]

use futures_signals::signal::{Mutable, MutableSignalCloned, Signal};
use std::{
    pin::Pin,
    task::{Context, Poll},
};

pub struct Mutable3<A, B, C>(
    (MutableSignalCloned<A>, Mutable<A>),
    (MutableSignalCloned<B>, Mutable<B>),
    (MutableSignalCloned<C>, Mutable<C>),
)
where
    A: Clone,
    B: Clone,
    C: Clone;
impl<A, B, C> Mutable3<A, B, C>
where
    A: Clone,
    B: Clone,
    C: Clone,
{
    pub fn new(a: Mutable<A>, b: Mutable<B>, c: Mutable<C>) -> Self {
        Mutable3(
            (a.signal_cloned(), a),
            (b.signal_cloned(), b),
            (c.signal_cloned(), c),
        )
    }
}
impl<A, B, C> Signal for Mutable3<A, B, C>
where
    A: Clone,
    B: Clone,
    C: Clone,
{
    type Item = (A, B, C);
    fn poll_change(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        let a = Pin::new(&mut self.0 .0).poll_change(cx);
        let b = Pin::new(&mut self.1 .0).poll_change(cx);
        let c = Pin::new(&mut self.2 .0).poll_change(cx);
        let mut changed = false;
        let left_done = match a {
            Poll::Ready(None) => true,
            Poll::Ready(_) => {
                changed = true;
                false
            }
            Poll::Pending => false,
        };
        let middle_done = match b {
            Poll::Ready(None) => true,
            Poll::Ready(_) => {
                changed = true;
                false
            }
            Poll::Pending => false,
        };
        let right_done = match c {
            Poll::Ready(None) => true,
            Poll::Ready(_) => {
                changed = true;
                false
            }
            Poll::Pending => false,
        };
        if changed {
            Poll::Ready(Some((
                self.0 .1.get_cloned(),
                self.1 .1.get_cloned(),
                self.2 .1.get_cloned(),
            )))
        } else if left_done && middle_done && right_done {
            Poll::Ready(None)
        } else {
            Poll::Pending
        }
    }
}
pub struct Mutable4<A, B, C, D>(
    (MutableSignalCloned<A>, Mutable<A>),
    (MutableSignalCloned<B>, Mutable<B>),
    (MutableSignalCloned<C>, Mutable<C>),
    (MutableSignalCloned<D>, Mutable<D>),
)
where
    A: Clone,
    B: Clone,
    C: Clone,
    D: Clone;
impl<A, B, C, D> Mutable4<A, B, C, D>
where
    A: Clone,
    B: Clone,
    C: Clone,
    D: Clone,
{
    pub fn new(a: Mutable<A>, b: Mutable<B>, c: Mutable<C>, d: Mutable<D>) -> Self {
        Mutable4(
            (a.signal_cloned(), a),
            (b.signal_cloned(), b),
            (c.signal_cloned(), c),
            (d.signal_cloned(), d),
        )
    }
}
impl<A, B, C, D> Signal for Mutable4<A, B, C, D>
where
    A: Clone,
    B: Clone,
    C: Clone,
    D: Clone,
{
    type Item = (A, B, C, D);
    fn poll_change(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        let a = Pin::new(&mut self.0 .0).poll_change(cx);
        let b = Pin::new(&mut self.1 .0).poll_change(cx);
        let c = Pin::new(&mut self.2 .0).poll_change(cx);
        let d = Pin::new(&mut self.3 .0).poll_change(cx);
        let mut changed = false;
        let left_done = match a {
            Poll::Ready(None) => true,
            Poll::Ready(_) => {
                changed = true;
                false
            }
            Poll::Pending => false,
        };
        let left_middle_done = match b {
            Poll::Ready(None) => true,
            Poll::Ready(_) => {
                changed = true;
                false
            }
            Poll::Pending => false,
        };
        let right_middle_done = match c {
            Poll::Ready(None) => true,
            Poll::Ready(_) => {
                changed = true;
                false
            }
            Poll::Pending => false,
        };
        let right_done = match d {
            Poll::Ready(None) => true,
            Poll::Ready(_) => {
                changed = true;
                false
            }
            Poll::Pending => false,
        };
        if changed {
            Poll::Ready(Some((
                self.0 .1.get_cloned(),
                self.1 .1.get_cloned(),
                self.2 .1.get_cloned(),
                self.3 .1.get_cloned(),
            )))
        } else if left_done && left_middle_done && right_middle_done && right_done {
            Poll::Ready(None)
        } else {
            Poll::Pending
        }
    }
}
