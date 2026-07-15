#![allow(unused)]

#[cfg(feature = "profiling")]
mod imp {
    use std::cell::RefCell;
    use std::ffi::CStr;
    use std::panic::Location;
    use std::sync::OnceLock;
    use tracy_client::{Client, Span};

    fn client() -> &'static Client {
        static CLIENT: OnceLock<Client> = OnceLock::new();
        CLIENT.get_or_init(Client::start)
    }

    thread_local! {
        static OPEN: RefCell<Vec<Span>> = const { RefCell::new(Vec::new()) };
    }

    #[inline]
    pub fn event(name: &str) {
        client().message(&name, 0);
    }

    #[inline]
    #[track_caller]
    pub fn interval_begin(name: &str, _id: u64) {
        let loc = Location::caller();
        let span = client().clone().span_alloc(
            Some(&name),
            "",
            loc.file(),
            loc.line(),
            0,
        );
        OPEN.with(|s| s.borrow_mut().push(span));
    }

    #[inline]
    pub fn interval_end(_name: &str, _id: u64) {
        OPEN.with(|s| {
            s.borrow_mut().pop();
        });
    }
}

#[cfg(not(feature = "profiling"))]
mod imp {
    #[inline]
    pub fn event(_name: &str) {}
    #[inline]
    pub fn interval_begin(_name: &str, _id: u64) {}
    #[inline]
    pub fn interval_end(_name: &str, _id: u64) {}
}

pub use imp::{event, interval_begin, interval_end};
