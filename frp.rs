use extra::dlist::DList;
use weak::{Weak, Strong};
use std::rc::Rc;
use util::FnMut;
use std::cell::RefCell;
use extra::container::Deque;

type Action<T> = ~FnMut:<(T,), ()>;

type Subscribers<T> = RefCell<DList<Action<T>>>;

#[deriving(Clone)]
pub struct EventStream<T> {
    priv subs: Weak<Subscribers<T>>
}

struct Closure_MergedWith<T> { gen: EventGenerator<T> }

impl<T: Clone> FnMut<(T,), ()> for Closure_MergedWith<T> {
    fn call(&mut self, (args,): (T,)) {
        self.gen.generate(args);
    }
}

impl<T: Clone> EventStream<T> {
    pub fn merged_with(&self, other: &EventStream<T>) -> EventStream<T> {
        let gen = EventGenerator::new();
        self.react(~Closure_MergedWith { gen: gen.clone() } as Action<T>);
        other.react(~Closure_MergedWith { gen: gen.clone() } as Action<T>);
        return gen.stream();
    }
}

impl<T> EventStream<T> {
    pub fn react(&self, sub: Action<T>) {
        match self.subs.upgrade() {
            Some(strong) => {
                let mut x = strong.borrow().borrow_mut();
                x.get().push_back(sub)
            }
            None => { }
        }
    }
}

#[deriving(Clone)]
pub struct EventGenerator<T> {
    priv subs: Strong<Subscribers<T>>
}
impl<T> EventGenerator<T> {
    pub fn new() -> EventGenerator<T> {
        EventGenerator { subs: Strong::new(RefCell::new(DList::new())) }
    }

    pub fn stream(&self) -> EventStream<T> {
        EventStream { subs: self.subs.downgrade() }
    }
}

impl<T: Clone> EventGenerator<T> {
    pub fn generate(&self, arg: T) {
        let mut x = self.subs.borrow().borrow_mut();
        for sub in x.get().mut_iter() {
            sub.call((arg.clone(),));
        }
    }
}

#[deriving(Clone)]
pub struct Variable<T> {
    priv value:   Rc<T>,
    priv changes: EventStream<T>
}

#[deriving(Clone)]
pub struct VariableCell<T> {
    priv generator: EventGenerator<T>,
    priv value:     Variable<T>
}
