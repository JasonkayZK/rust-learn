pub mod gui;

#[cfg(test)]
mod tests {
    use crate::gui::{Button, Draw, Screen};

    // Another Draw implementation!
    struct SelectBox {
        pub width: u32,
        pub height: u32,
        pub options: Vec<String>,
    }

    impl Draw for SelectBox {
        fn draw(&self) {
            println!("draw a select-box on screen")
        }
    }

    #[test]
    fn test_screen_run() {
        let screen = Screen {
            components: vec![
                Box::new(SelectBox {
                    width: 75,
                    height: 10,
                    options: vec![
                        String::from("Yes"),
                        String::from("Maybe"),
                        String::from("No"),
                    ],
                }),
                Box::new(Button {
                    width: 50,
                    height: 10,
                    label: String::from("Ok"),
                }),
            ],
        };
        screen.run();
    }
}
