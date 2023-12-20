use tokio::sync::Notify;

use std::collections::VecDeque;
use std::sync::Mutex;

pub struct DispatchChannel<T> {
    messages: Mutex<VecDeque<T>>,
    notify_on_sent: Notify,
}

impl<T> DispatchChannel<T> {
    pub fn new() -> Self {
        Self {
            messages: Mutex::new(VecDeque::new()),
            notify_on_sent: Notify::new()
        }
    }

    pub fn send(&self, msg: T) {
        let mut locked_queue = self.messages.lock().unwrap();
        locked_queue.push_back(msg);
        drop(locked_queue);

        // Send a notification to one of the calls currently
        // waiting in a call to `recv`.
        self.notify_on_sent.notify_one();
    }

    pub fn try_recv(&self) -> Option<T> {
        let mut locked_queue = self.messages.lock().unwrap();
        locked_queue.pop_front()
    }

    pub async fn recv(&self) -> T {
        let future = self.notify_on_sent.notified();
        tokio::pin!(future);

        loop {
            // Make sure that no wakeup is lost if we get
            // `None` from `try_recv`.
            future.as_mut().enable();

            if let Some(msg) = self.try_recv() {
                return msg;
            }

            // Wait for a call to `notify_one`.
            //
            // This uses `.as_mut()` to avoid consuming the future,
            // which lets us call `Pin::set` below.
            future.as_mut().await;

            // Reset the future in case another call to
            // `try_recv` got the message before us.
            future.set(self.notify_on_sent.notified());
        }
    }
}