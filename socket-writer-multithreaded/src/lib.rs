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
    waker: AtomicPtr<Waker>,
    exit: AtomicBool,
    receiver: Mutex<Receiver>,
    messages_buffer: Mutex<Vec<usize>>,

    flush_limit: usize,
    flush_interval: Duration,
}

impl SharedState {
    fn push_ticks(&self, i: usize) {
        self.messages_buffer.lock().unwrap().push(i);
        if i % self.flush_limit == 0 {
            let ptr = self.waker.swap(std::ptr::null_mut(), Ordering::Release);
            if !ptr.is_null() {
                unsafe { Box::from_raw(ptr) }.wake();
            }
        }
    }

    async fn receive_data(&self) {
        loop {
            let res = tokio::time::timeout(self.flush_interval, WaitForBuffer(self)).await;
            if self.exit.load(Ordering::Relaxed) {
                return;
            }

            if res.is_err() {
                self.receiver.lock().unwrap().keepalive();
                continue;
            }
            let data: Vec<_> = self.messages_buffer.lock().unwrap().drain(..).collect();
            self.receiver.lock().unwrap().send_data(&data);
        }
    }
}

struct WaitForBuffer<'a>(&'a SharedState);

impl<'a> Future for WaitForBuffer<'a> {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let me = self.get_mut();
        let previous_waker = me.0.waker.swap(
            Box::into_raw(Box::new(cx.waker().clone())),
            Ordering::Acquire,
        );
        if previous_waker.is_null() {
            return Poll::Pending;
        }
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
            waker: AtomicPtr::new(null_mut()),
            exit: AtomicBool::new(false),
            messages_buffer: Mutex::new(Vec::new()),
            receiver: Mutex::new(Receiver::new()),
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
                sh.exit.store(true, Ordering::Relaxed);
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
