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
}

glasses_harness!(
    SimpleTestHarness,
    ExpectedValue,
    ExpectedValue{
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

glasses_test!(
    SimpleTestHarness,
    empty_test_passes,
    value 0
);

glasses_test!(
    SimpleTestHarness,
    simple_equality_passes,
    value 0,
    expected 0
);

glasses_test!(
    SimpleTestHarness,
    missing_precondition_panics,
    [should_panic]
);

glasses_test!(
    SimpleTestHarness,
    simple_inequality_panics,
    [should_panic],
    value 1,
    expected 0
);
