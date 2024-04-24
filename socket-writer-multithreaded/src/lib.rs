use core::task::{Context, Poll, Waker};
use std::{
    future::Future,
    pin::Pin,
    time::Duration,
    sync::{Mutex, atomic::{AtomicBool, AtomicPtr, Ordering}},
};
mod receiver;

use receiver::Receiver;

struct SharedState {

    flush_limit: usize,
    flush_interval: Duration,
}

impl SharedState {
    fn push_ticks(&self, i: usize) {
        if i % self.flush_limit == 0 {
        }
    }

    async fn receive_data(&self) {
        loop {
        }
    }
}

struct WaitForBuffer<'a>(&'a SharedState);

impl<'a> Future for WaitForBuffer<'a> {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready(())
    }
}

#[cfg(test)]
mod tests {
    use core::time;
    use std::{ptr::null_mut, sync::Arc};

    use super::*;

    #[test]
    fn it_works() {
        let rt = tokio::runtime::Runtime::new().unwrap();

        let sh = Arc::new(SharedState {
            flush_limit: 100,
            flush_interval: time::Duration::from_millis(10),
        });

        rt.spawn({
            let sh = sh.clone();
            async move {
                for i in 0..10000 {
                    sh.push_ticks(i);
                    if i % 500 == 0 {
                        tokio::time::sleep(Duration::from_millis(20)).await;
                    }
                }
                // sh.exit.store(true, Ordering::Relaxed);
            }
        });

        rt.spawn({
            let sh = sh.clone();
            async move {
                sh.receive_data().await;
            }
        });

        rt.block_on(async{});
    }
}
