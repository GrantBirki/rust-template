use rust_template::{add, greet, subtract, version_info};
use speculate2::speculate;

speculate! {
    describe "basic math" {
        it "adds two numbers" {
            assert_eq!(add(2, 3), 5);
        }

        it "subtracts two numbers" {
            assert_eq!(subtract(5, 3), 2);
        }
    }

    describe "greeting" {
        it "greets by name" {
            assert_eq!(greet("Grant", false, 1), "Hello, Grant!");
        }

        it "greets multiple times" {
            assert_eq!(greet("Codex", false, 2), "Hello, Codex!\nHello, Codex!");
        }

        it "shouts when requested" {
            assert_eq!(greet("world", true, 1), "HELLO, WORLD!");
        }
    }

    describe "version metadata" {
        it "includes package name and version" {
            let info = version_info();
            assert_eq!(info.name, env!("CARGO_PKG_NAME"));
            assert_eq!(info.version, env!("CARGO_PKG_VERSION"));
        }
    }
}
