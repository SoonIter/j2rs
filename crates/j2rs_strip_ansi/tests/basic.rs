use j2rs_strip_ansi::strip_ansi;
use picocolors::bg_black;

macro_rules! make_test {
  ( $( $name:ident: [$left:expr, $right:expr]),* ) => {
    $(
        #[test]
        fn $name() {
          let left = $left;
          let left = strip_ansi(left.as_ref());
          let right = $right;
          assert_eq!(left, right);
        }
    )*
  }
}

make_test! {
  basic: ["\u{1B}[0m\u{1B}[4m\u{1B}[42m\u{1B}[31mfoo\u{1B}[39m\u{1B}[49m\u{1B}[24mfoo\u{1B}[0m", "foofoo"],
  from_ls: ["\u{1B}[00;38;5;244m\u{1B}[m\u{1B}[00;38;5;33mfoo\u{1B}[m", "foo"],
  picocolors: [bg_black("hello"), "hello".to_string()]
}
