use glasses::*;

struct ExpectedValue {
    value: Option<i32>,
    expected: Option<i32>,
}

impl ExpectedValue {
    pub fn value(mut self, value: i32) -> Self {
        self.value = Some(value);
        self
    }
    pub fn expected(mut self, expected: i32) -> Self {
        self.expected = Some(expected);
        self
    }
    pub fn does_nothing(self) -> Self {
        self
    }
}

glasses_harness!(
    SimpleTestHarness,
    ExpectedValue,
    ExpectedValue {
        value: None,
        expected: None,
    },
    |case: ExpectedValue| {
        assert!(case.value.is_some(), "Expected a value");
        if let Some(expected) = case.expected {
            assert_eq!(case.value.unwrap(), expected);
        }
    }
);

#[macro_export]
macro_rules! make_test {
    ($name: ident, $value: expr, $expected: expr $(, $mod: ident $($arg: expr)*)*) => {
        glasses::glasses_test!(
            SimpleTestHarness,
            $name,
            value $value,
            expected $expected
            $(, $mod $( $arg )* )*
        );
    }
}

#[macro_export]
macro_rules! make_failing_test {
    ($name: ident, $value: expr, $expected: expr $(, $mod: ident $($arg: expr)*)*) => {
        glasses::glasses_test!(
            SimpleTestHarness,
            $name,
            [should_panic],
            value $value,
            expected $expected
            $(, $mod $( $arg )* )*
        );
    }
}

make_test!(macro_equality_passes, 0, 0);

make_failing_test!(macro_inequality_panics, 1, 0);

make_test!(
    macro_equality_passes_and_supports_extra_builder_mods,
    0,
    0,
    does_nothing
);

make_failing_test!(
    macro_inequality_panics_and_supports_extra_builder_mods,
    1,
    0,
    does_nothing
);
