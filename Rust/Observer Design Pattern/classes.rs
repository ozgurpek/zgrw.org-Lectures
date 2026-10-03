// Rust version of classes.h + classes.cc
//
// In C++ the Subject keeps raw `Observer *` pointers and every Observer keeps a
// raw `Subject *`. In Rust we share ownership with Rc (reference counting):
// - the Subject owns its observers through Rc<dyn Observer>
// - each observer points back to the Subject with a Weak reference, so the two
//   do not keep each other alive forever (a reference cycle)
// Cell / RefCell let us change `val` and the observer list through a shared reference.

use std::cell::{Cell, RefCell};
use std::rc::{Rc, Weak};

pub struct Subject {
    val: Cell<i32>,
    observers: RefCell<Vec<Rc<dyn Observer>>>,
}

impl Subject {
    pub fn new() -> Rc<Subject> {
        Rc::new(Subject {
            val: Cell::new(1),
            observers: RefCell::new(Vec::new()),
        })
    }

    pub fn set_val(&self, v: i32) {
        self.val.set(v);
        self.notify();
    }

    pub fn get_val(&self) -> i32 {
        self.val.get()
    }

    pub fn subscribe(&self, o: Rc<dyn Observer>) {
        self.observers.borrow_mut().push(o);
    }

    fn notify(&self) {
        for o in self.observers.borrow().iter() {
            o.update();
        }
    }
}

// The abstract C++ base class becomes a trait (the pure virtual update())...
pub trait Observer {
    fn update(&self);
}

// ...plus a struct with the data members every observer shares.
pub struct ObserverBase {
    denom: i32,
    model: Weak<Subject>,
}

impl ObserverBase {
    fn new(model: &Rc<Subject>, denom: i32) -> ObserverBase {
        ObserverBase {
            denom,
            model: Rc::downgrade(model),
        }
    }

    pub fn get_subject(&self) -> Rc<Subject> {
        self.model.upgrade().expect("subject was dropped")
    }
}

pub struct DivObserver {
    base: ObserverBase,
}

impl DivObserver {
    // like the C++ constructor, creating an observer subscribes it to the model
    pub fn new(model: &Rc<Subject>, denom: i32) -> Rc<DivObserver> {
        let o = Rc::new(DivObserver {
            base: ObserverBase::new(model, denom),
        });
        model.subscribe(o.clone());
        o
    }
}

impl Observer for DivObserver {
    fn update(&self) {
        let v = self.base.get_subject().get_val();
        let denom = self.base.denom;
        println!("{} / {} = {}", v, denom, v / denom);
    }
}

pub struct ModObserver {
    base: ObserverBase,
}

impl ModObserver {
    // like the C++ constructor, creating an observer subscribes it to the model
    pub fn new(model: &Rc<Subject>, denom: i32) -> Rc<ModObserver> {
        let o = Rc::new(ModObserver {
            base: ObserverBase::new(model, denom),
        });
        model.subscribe(o.clone());
        o
    }
}

impl Observer for ModObserver {
    fn update(&self) {
        let v = self.base.get_subject().get_val();
        let denom = self.base.denom;
        println!("{} % {} = {}", v, denom, v % denom);
    }
}
