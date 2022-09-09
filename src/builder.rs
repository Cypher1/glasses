pub trait Builder: Default {
    type Target;

    fn build(self) -> Self::Target;
}

pub trait Buildable {
    type Builder;

    fn builder() -> Self::Builder;
}

#[macro_export]
macro_rules! glasses_builder_validator{
    ($self: expr, $field: ident, $value: expr, $type: ty, [drop] $([$meta: ident $($arg: expr)*])*) => {{
        glasses_builder_validator!($self, $field, $value, $type, $([$meta $($arg)*])*)
    }};
    ($self: expr, $field: ident, $value: expr, $type: ty, [with $pred: expr] $([$meta: ident $($arg: expr)*])*) => {{
        let value = $value;
        assert!($pred($self), "predicate associated with {} failed:\n{}", stringify!($field), stringify!($pred));
        glasses_builder_validator!($self, $field, value, $type, $([$meta $($arg)*])*)
    }};
    ($self: expr, $field: ident, $value: expr, $type: ty, [at_least $len: expr] $([$meta: ident $($arg: expr)*])*) => {{
        let value = $value;
        let len = value.len();
        assert!(len >= $len, "{}: expected at least {} value but found {}", stringify!($field), $len, len);
        glasses_builder_validator!($self, $field, value, $type, $([$meta $($arg)*])*)
    }};
    ($self: expr, $field: ident, $value: expr, $type: ty, [many] $([$meta: ident $($arg: expr)*])*) => {{
        $value.iter().for_each(|value| glasses_builder_validator!($self, $field, value, $type, $([$meta $($arg)*])*));
    }};
    ($self: expr, $field: ident, $value: expr, $type: ty, [optional] $([$meta: ident $($arg: expr)*])*) => {{
        if let Some(value) = $value {
            Some(glasses_builder_validator!($self, $field, value, $type, $([$meta $($arg)*])*))
        } else {
            None
        }
    }};
    ($self: expr, $field: ident, $value: expr, $type: ty, [set_once] $([$meta: ident $($arg: expr)*])*) => {{
        let value = ($value).expect("Should have been set.");
        glasses_builder_validator!($self, $field, value, $type, $([$meta $($arg)*])*)
    }};
    ($self: expr, $field: ident, $value: expr, $type: ty,) => {
        {}
    };
}

#[macro_export]
macro_rules! glasses_builder_value{
    ($field: ident, $value: expr, $type: ty, [drop] $([$meta: ident $($arg: expr)*])*) => {{
        glasses_builder_value!($field, $value, $type, $([$meta $($arg)*])*)
    }};
    ($field: ident, $value: expr, $type: ty, [with $pred: expr] $([$meta: ident $($arg: expr)*])*) => {{
        glasses_builder_value!($field, $value, $type, $([$meta $($arg)*])*)
    }};
    ($field: ident, $value: expr, $type: ty, [at_least $len: expr] $([$meta: ident $($arg: expr)*])*) => {{
        glasses_builder_value!($field, $value, $type, $([$meta $($arg)*])*)
    }};
    ($field: ident, $value: expr, $type: ty, [many] $([$meta: ident $($arg: expr)*])*) => {{
        let mut value = $value;
        value.drain(0..).map(|item| glasses_builder_value!($field, item, $type, $([$meta $($arg)*])*)).collect()
    }};
    ($field: ident, $value: expr, $type: ty, [optional] $([$meta: ident $($arg: expr)*])*) => {{
        if let Some(value) = $value {
            Some(glasses_builder_value!($field, value, $type, $([$meta $($arg)*])*))
        } else {
            None
        }
    }};
    ($field: ident, $value: expr, $type: ty, [set_once] $([$meta: ident $($arg: expr)*])*) => {{
        let value = ($value).expect("Should have been set.");
        glasses_builder_value!($field, value, $type, $([$meta $($arg)*])*)
    }};
    ($field: ident, $value: expr, $type: ty,) => {
        $value
    };
}

#[macro_export]
macro_rules! glasses_builder_field_setter{
    ($field: ident, $value: expr, $new_value: expr, $type: ty [drop] $([$meta: ident $($arg: expr)*])*) => {{
        glasses_builder_field_setter!($field, $value, $new_value, $type $([$meta $($arg)*])*)
    }};
    ($field: ident, $value: expr, $new_value: expr, $type: ty [with $pred: expr] $([$meta: ident $($arg: expr)*])*) => {{
        glasses_builder_field_setter!($field, $value, $new_value, $type $([$meta $($arg)*])*)
    }};
    ($field: ident, $value: expr, $new_value: expr, $type: ty [at_least $len: expr] $([$meta: ident $($arg: expr)*])*) => {{
        glasses_builder_field_setter!($field, $value, $new_value, $type $([$meta $($arg)*])*)
    }};
    ($field: ident, $value: expr, $new_value: expr, $type: ty [many] $([$meta: ident $($arg: expr)*])*) => {{
        let mut value = $value;
        value.push(glasses_builder_field_setter!($field, $value, $new_value, $type $([$meta $($arg)*])*));
        value
    }};
    ($field: ident, $value: expr, $new_value: expr, $type: ty [optional] $([$meta: ident $($arg: expr)*])*) => {{
        assert!($value.is_none(), "Field {} has already been set.", stringify!($field));
        Some(glasses_builder_field_setter!($field, $value, $new_value, $type $([$meta $($arg)*])*))
    }};
    ($field: ident, $value: expr, $new_value: expr, $type: ty [set_once] $([$meta: ident $($arg: expr)*])*) => {{
        assert!($value.is_none(), "Field {} has already been set but is marked [set_once].", stringify!($field));
        glasses_builder_field_setter!($field, $value, $new_value, $type $([$meta $($arg)*])*).expect("Field should be set")
    }};
    ($field: ident, $value: expr, $new_value: expr, $type: ty) => {
        $new_value
    };
}

#[macro_export]
macro_rules! glasses_builder_field_type {
    ($type: ty [drop] $([$meta: ident $($arg: expr)*])*) => {
        glasses_builder_field_type!($type $([$meta $($arg)*])*)
    };
    ($type: ty [with $pred: expr] $([$meta: ident $($arg: expr)*])*) => {
        glasses_builder_field_type!($type $([$meta $($arg)*])*)
    };
    ($type: ty [at_least $len: expr] $([$meta: ident $($arg: expr)*])*) => {
        glasses_builder_field_type!($type $([$meta $($arg)*])*)
    };
    ($type: ty [many] $([$meta: ident $($arg: expr)*])*) => {
        Vec<glasses_builder_field_type!($type $([$meta $($arg)*])*)>
    };
    ($type: ty [optional] $([$meta: ident $($arg: expr)*])*) => {
        Option<glasses_builder_field_type!($type $([$meta $($arg)*])*)>
    };
    ($type: ty [set_once] $([$meta: ident $($arg: expr)*])*) => {
        Option< glasses_builder_field_type!($type $([$meta $($arg)*])*)>
    };
    ($type: ty) => {
        $type
    };
}

#[macro_export]
macro_rules! glasses_builder {
    ($ty: ident
     $(,
        $field: ident $field_type: ty $([$meta: ident $($arg: expr)*])*
     )*
    ) => {
        use paste::paste;
        use $crate::builder::{Builder, Buildable};
        paste!{
            #[derive(Default)]
            struct [< $ty Builder >] {
                $(
                    $field: glasses_builder_field_type!($field_type $([$meta $($arg)*])*),
                )*
            }

            impl [< $ty Builder >] {
                $(
                    fn $field(mut self, value: $field_type) -> Self {
                        self.$field = glasses_builder_field_setter!($field, self.$field, value, $field_type $([$meta $($arg)*])*);
                        self
                    }
                )*
            }

            impl Into<$ty> for [< $ty Builder >] {
                fn into(self) -> $ty {
                    self.build()
                }
            }

            impl Builder for [< $ty Builder >] {
                type Target = $ty;

                fn build(self) -> Self::Target {
                    $(
                        glasses_builder_validator!(&self, $field, &self.$field, $field_type, $([$meta $($arg)*])*);
                    )*
                    Self::Target {
                        $(
                            $field: glasses_builder_value!($field, self.$field, $field_type, $([$meta $($arg)*])*),
                        )*
                    }
                }
            }

            impl Buildable for $ty {
                type Builder = [< $ty Builder >];

                fn builder() -> Self::Builder {
                    Self::Builder::default()
                }
            }
        }
    };
}
