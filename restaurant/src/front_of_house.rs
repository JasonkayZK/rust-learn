// 餐厅前厅，用于吃饭
pub mod hosting {
    pub fn add_to_waitlist() {}

    fn seat_at_table() {}
}

pub mod serving {
    fn take_order() {}

    fn serve_order() {
        self::back_of_house::cook_order()
    }

    fn take_payment() {}

    // 厨房模块
    mod back_of_house {
        fn fix_incorrect_order() {
            cook_order();
            super::serve_order();
        }

        pub fn cook_order() {}
    }
}
