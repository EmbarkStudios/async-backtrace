use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};
use std::marker::PhantomPinned;

use crate::frame::Frame;
use crate::location::Location;

use pin_project_lite::pin_project;

pin_project! {
    /// Includes a given future in taskdumps.
    pub struct Traced<F> {
        // The wrapped future.
        #[pin]
        future: F,
        // Metadata about the wrapped future.
        #[pin]
        frame: Frame,
        // True if the future hasn't been polled yet.
        polled: bool,
        _pinned: PhantomPinned,
    }
}

unsafe impl<F: Send> Send for Traced<F> {}
unsafe impl<F: Sync> Sync for Traced<F> {}
impl<F: core::panic::UnwindSafe> core::panic::UnwindSafe for Traced<F> {}

impl<F> Traced<F> {
    /// Include the given `future` in taskdumps with the given `location`.
    pub fn new(future: F, location: Location) -> Self {
        Self {
            future,
            frame: Frame::new(location),
            polled: false,
            _pinned: PhantomPinned,
        }
    }
}

impl<F> Future for Traced<F>
where
    F: Future,
{
    type Output = <F as Future>::Output;

    #[track_caller]
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as Future>::Output> {
        let this = self.project();
        this.frame.in_scope(|| this.future.poll(cx))
    }
}
