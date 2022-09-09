use glasses::*;

struct ExpectedValue {
    value: Vec<i32>,
    expected: Option<i32>,
    #[allow(dead_code)]
    other: Option<i32>
}

glasses_builder!(
    ExpectedValue,
    value i32 [at_least 1] [many],
    expected i32 [optional],
    other i32 [optional] [drop]
    [with |case: &ExpectedValueBuilder| {
        case.other.is_none() ||
        case.expected.is_none() ||
        case.other.unwrap() > case.expected.unwrap()
    }]
);
    // [func] does_nothing

glasses_harness!(
    MacroForCase,
    ExpectedValueBuilder,
    ExpectedValue::builder(),
    |builder: ExpectedValueBuilder| {
        let case = builder.build();
        assert!(!case.value.is_empty(), "Expected at least one value");
        if let Some(expected) = case.expected {
            for value in case.value {
                assert_eq!(value, expected);
            }
        }
    }
);

glasses_test!(
    MacroForCase,
    no_expectation_test_passes,
    value 0
);

glasses_test!(
    MacroForCase,
    simple_equality_passes,
    value 0,
    expected 0
);

glasses_test!(
    MacroForCase,
    multiple_equality_passes,
    value 0,
    value 0,
    expected 0
);

glasses_test!(
    MacroForCase,
    missing_precondition_panics,
    [should_panic]
);

glasses_test!(
    MacroForCase,
    simple_inequality_panics,
    [should_panic],
    value 1,
    expected 0
);

glasses_test!(
    MacroForCase,
    with_predicate_can_succeed,
    value 0,
    expected 0,
    other 3
);

glasses_test!(
    MacroForCase,
    with_predicate_can_fail_test,
    [should_panic],
    value 1,
    expected 0,
    other 3
);

glasses_test!(
    MacroForCase,
    with_predicate_can_fail_builder,
    [should_panic],
    value 0,
    expected 0,
    other 0
);
