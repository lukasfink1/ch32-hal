#[cfg(feature = "use-wakers")]
pub use embassy_sync::waitqueue;

#[cfg(not(feature = "use-wakers"))]
pub mod waitqueue {
    use core::task::Waker;

    pub struct AtomicWaker {
        _private: (),
    }

    impl AtomicWaker {
        pub const fn new() -> Self {
            Self { _private: () }
        }

        pub fn register(&self, _w: &Waker) {}
        pub fn wake(&self) {}
    }
}
