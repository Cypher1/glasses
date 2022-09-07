pub trait Harness {
    type Builder;
    fn new_builder() -> Self::Builder;
    fn run_test(case: Self::Builder);
}

#[macro_export]
macro_rules! glasses_make_case_helper {
    ($harness_name: ty, $acc: expr) => {{
        use $crate::Harness;
        let case = <$harness_name>::new_builder();
        ($acc)(case)
    }};
    ($harness_name: ty, $acc: expr, $head_mod: ident $($head_arg: expr)* $(, $mod: ident $($arg: expr)*)*) => {{
        $crate::glasses_make_case_helper!(
            $harness_name,
            |case: <$harness_name as Harness>::Builder | $acc(case).$head_mod($($head_arg , )*)
            $(, $mod $( $arg )* )*
        )
    }};
}

#[macro_export]
macro_rules! glasses_make_case {
    ($harness_name: ty $(, $mod: ident $($arg: expr)*)*) => {
        $crate::glasses_make_case_helper!($harness_name, |case: <$harness_name as Harness>::Builder| case $(, $mod $( $arg )* )* )
    }
}

#[macro_export]
macro_rules! glasses_harness {
    ($harness_name: ident, $builder_type: ty, $test_body: expr) => {
        $crate::glasses_harness!($harness_name, $builder_type, <$builder_type>::default(), $test_body);
    };
    ($harness_name: ident, $builder_type: ty, $builder: expr, $test_body: expr) => {
        pub struct $harness_name;

        impl glasses::Harness for $harness_name {
            type Builder = $builder_type;
            fn new_builder() -> $builder_type {
                $builder
            }
            fn run_test(case: $builder_type) {
                $test_body(case)
            }
        }
    }
}

#[macro_export]
macro_rules! glasses_test {
    (
        $harness_name: ty,
        $name: ident
        $(, [ $attr: meta ] )*
        $(, $mod: ident $($arg: expr)*)*
    ) => {
        #[test]
        $(#[$attr])*
        fn $name() {
            use $crate::Harness;
            let case: <$harness_name as Harness>::Builder = $crate::glasses_make_case!($harness_name $(, $mod $( $arg )* )* );
            <$harness_name>::run_test(case)
        }
    }
}
