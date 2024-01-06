use tokio::sync::Notify;
use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use crate::message::DispatchMessage;


pub struct DispatchChannel {
    messages: Mutex<VecDeque<DispatchMessage>>,
    notify_on_sent: Notify,
    terminate: AtomicBool
}

impl DispatchChannel {
    pub fn send(&self, msg: DispatchMessage) {
        let mut locked_queue = self.messages.lock().unwrap();
        locked_queue.push_back(msg);
        drop(locked_queue);

        // Send a notification to one of the calls currently
        // waiting in a call to `recv`.
        self.notify_on_sent.notify_one();
    }

    pub fn terminate(&self) {
        self.terminate.store(true, Ordering::Relaxed)
    }

    fn try_recv(&self) -> Option<DispatchMessage> {
        let mut locked_queue = self.messages.lock().unwrap();
        locked_queue.pop_front()
    }

    pub async fn recv(&self) -> Option<DispatchMessage> {
        let future = self.notify_on_sent.notified();
        tokio::pin!(future);

        while !self.terminate.load(Ordering::Relaxed) {
            // Make sure that no wakeup is lost if we get
            // `None` from `try_recv`.
            future.as_mut().enable();

            if let Some(msg) = self.try_recv() {
                return Some(msg);
            }

            println!("***DispatchChannel recv waiting for notification ...");
            // Wait for a call to `notify_one`.
            //
            // This uses `.as_mut()` to avoid consuming the future,
            // which lets us call `Pin::set` below.
            future.as_mut().await;

            // Reset the future in case another call to
            // `try_recv` got the message before us.
            future.set(self.notify_on_sent.notified());
        }

        return None
    }
}

impl Default for DispatchChannel {
    fn default() -> Self {
        Self {
            messages: Mutex::new(VecDeque::new()),
            notify_on_sent: Notify::new(),
            terminate: AtomicBool::new(false),
        }
    }
}