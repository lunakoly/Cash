pub mod visitors;

use crate::rendering::*;

/// A single field of a struct.
pub struct FieldInfo {
    pub name: String,
    pub proto: String,
}

const FIELD_TEMPLATE: &'static str = "
    pub $$: $$,
";

/// Returns the string representation
/// of a struct field.
pub fn render_field(field: &FieldInfo) -> String {
    return render(FIELD_TEMPLATE, 4, &[&field.name, &field.proto]);
}

const STRUCT_TEMPLATE: &'static str = "
    pub struct $$ {
    $$
    }
";

/// Returns the struct representation.
pub fn render_struct(name: &str, fields: &Vec<FieldInfo>) -> String {
    let mut pieces = vec!();

    for it in fields {
        pieces.push(render_field(it));
    }

    return render(STRUCT_TEMPLATE, 0, &[name, &pieces.join("\n")]);
}

const TRAIT_TEMPLATE: &'static str = "
    pub trait $$ {
    $$
    }
";

/// Returns the trait representation.
pub fn render_trait(name: &str, methods: &str) -> String {
    return render(TRAIT_TEMPLATE, 0, &[name, methods]);
}

const IMPL_TEMPLATE: &'static str = "
    impl $$ for $$ {
    $$
    }
";

/// Returns the impl representation.
pub fn render_impl(trait_name: &str, struct_name: &str, methods: &str) -> String {
    return render(IMPL_TEMPLATE, 0, &[trait_name, struct_name, methods]);
}
