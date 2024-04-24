use core::task::{Context, Poll, Waker};
use std::{
    cell::{Cell, RefCell},
    future::Future,
    pin::Pin,
    time::Duration,
};
mod receiver;

use receiver::Receiver;

struct SharedState {
    waker: Cell<Option<Waker>>,
    exit: Cell<bool>,
    messages_buffer: RefCell<Vec<usize>>,

    flush_limit: usize,
    flush_interval: Duration,

    receiver: RefCell<Receiver>,
}

impl SharedState {
    fn push_ticks(&self, i: usize) {
        self.messages_buffer.borrow_mut().push(i);
        if i % self.flush_limit == 0 {
            if let Some(w) = self.waker.replace(None) {
                w.wake()
            }
        }
    }

    async fn receive_data(&self) {
        loop {
            let res = tokio::time::timeout(self.flush_interval, WaitForBuffer(self)).await;
            if self.exit.get() {
                return;
            }

            if res.is_err() {
                self.receiver.borrow_mut().keepalive();
                continue;
            }
            let data: Vec<_> = self.messages_buffer.borrow_mut().drain(..).collect();
            self.receiver.borrow_mut().send_data(&data);
        }
    }
}

struct WaitForBuffer<'a>(&'a SharedState);

impl<'a> Future for WaitForBuffer<'a> {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let me = self.get_mut();
        if me.0.waker.replace(Some(cx.waker().clone())).is_some() {
            return Poll::Pending;
        };
        Poll::Ready(())
    }
}

#[cfg(test)]
mod tests {
    use core::time;
    use std::rc::Rc;

    use super::*;

    #[test]
    fn it_works() {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        let sh = Rc::new(SharedState {
            waker: Cell::new(None),
            exit: Cell::new(false),
            messages_buffer: RefCell::new(Vec::new()),
            receiver: std::cell::RefCell::new(Receiver::new()),
            flush_limit: 100,
            flush_interval: time::Duration::from_millis(10),
        });

        let set = tokio::task::LocalSet::new();
        set.spawn_local({
            let sh = sh.clone();
            async move {
                for i in 0..10000 {
                    sh.push_ticks(i);
                    if i % 500 == 0 {
                        tokio::time::sleep(Duration::from_millis(20)).await;
                    }
                }
                sh.exit.replace(true);
            }
        });
        set.spawn_local({
            let sh = sh.clone();
            async move {
                sh.receive_data().await;
            }
        });

        rt.block_on(set);
    }
}
