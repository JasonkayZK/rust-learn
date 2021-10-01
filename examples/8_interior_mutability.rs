use std::cell::RefCell;

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T> where T: Messenger {
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;
        if percentage_of_max >= 0.75 && percentage_of_max < 0.9 {
            self.messenger.send("Warning: You've used up over 75% of your quota!");
        } else if percentage_of_max >= 0.9 && percentage_of_max < 1.0 {
            self.messenger.send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        }
    }
}

fn it_sends_an_over_75_percent_warning_message() {
    struct MockMessenger {
        // can not use immutable vec
        // sent_messages: Vec<String>,

        sent_messages: RefCell<Vec<String>>,
    }
    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger { sent_messages: RefCell::new(vec![] )}
        }
    }
    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            println!("send message: {}", msg);
            self.sent_messages.borrow_mut().push(String::from(msg));

            // Cause Panic：panicked at 'already borrowed: BorrowMutError'
            // let mut first_borrow = self.sent_messages.borrow_mut();
            // let mut second_borrow = self.sent_messages.borrow_mut();
            // first_borrow.push(String::from(msg));
            // second_borrow.push(String::from(msg));
        }
    }

    let mock_messenger = MockMessenger::new();
    let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
    limit_tracker.set_value(75);

    assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
}

fn main() {
    it_sends_an_over_75_percent_warning_message();
}
