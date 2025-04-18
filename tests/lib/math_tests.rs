use speculate2::speculate;
use rust_template::{add, subtract};

speculate! {
    describe "basic math" {
        it "adds two numbers" {
            assert_eq!(add(2, 3), 5);
        }

        it "subtracts two numbers" {
            assert_eq!(subtract(5, 3), 2);
        }
    }
}
