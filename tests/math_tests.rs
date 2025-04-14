use speculate::speculate;

speculate! {
    describe "basic math" {
        it "adds two numbers" {
            assert_eq!(2 + 2, 4);
        }

        it "multiplies numbers" {
            assert_eq!(3 * 3, 9);
        }
    }
}
