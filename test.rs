#![feature(panic_internals)]

fn main() {
    // println!("{}", []);
    assert_eq!(vec!["x"], vec![]);
    // vec!["x"] == vec![];
    // match (&vec!["x"], &vec![]) {
    //     (left_val, right_val) => {
    //         if !(*left_val == *right_val) {
    //             let kind = core::panicking::AssertKind::Eq;
    //             // The reborrows below are intentional. Without them, the stack slot for the
    //             // borrow is initialized even before the values are compared, leading to a
    //             // noticeable slow down.
    //             core::panicking::assert_failed(
    //                 kind,
    //                 &*left_val,
    //                 &*right_val,
    //                 core::option::Option::None,
    //             );
    //         }
    //     }
    // }
}
