#![allow(dead_code)]
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percent_of_max = self.value as f32 / self.max as f32;

        if percent_of_max > 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percent_of_max >= 0.8 {
            self.messenger
                .send("Urgent warning: You've used up over 80% of your quota!");
        } else if percent_of_max >= 0.66 {
            self.messenger
                .send("Warning: You've used up over 66% of your quota!");
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(message.to_string());
        }
    }

    #[test]
    fn it_sends_a_quota_warning() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        let message_contains_quota = mock_messenger
            .sent_messages
            .borrow()
            .get(0)
            .unwrap_or(&String::from(""))
            .contains("quota");

        assert!(message_contains_quota);
    }
}
