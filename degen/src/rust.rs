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
pub fn render_field(field: &FieldInfo, indent: usize) -> String {
    return render(FIELD_TEMPLATE, indent, &[&field.name, &field.proto]);
}

const STRUCT_TEMPLATE: &'static str = "
    pub struct $$ {
    $$
    }
";

/// Returns the struct representation.
pub fn render_struct(name: &str, fields: &Vec<FieldInfo>, indent: usize) -> String {
    let mut pieces = vec!();

    for it in fields {
        pieces.push(render_field(it, 4));
    }

    return render(STRUCT_TEMPLATE, indent, &[name, &pieces.join("\n")]);
}

const TRAIT_TEMPLATE: &'static str = "
    pub trait $$ {
    $$
    }
";

/// Returns the trait representation.
pub fn render_trait(name: &str, methods: &str, indent: usize) -> String {
    return render(TRAIT_TEMPLATE, indent, &[name, methods]);
}

const IMPL_TEMPLATE: &'static str = "
    impl $$ for $$ {
    $$
    }
";

/// Returns the impl representation.
pub fn render_impl(trait_name: &str, struct_name: &str, methods: &str, indent: usize) -> String {
    return render(IMPL_TEMPLATE, indent, &[trait_name, struct_name, methods]);
}

const MOD_TEMPLATE: &'static str = "
    pub mod $$ {
    $$
    }
";

/// Returns the impl representation.
pub fn render_mod(mod_name: &str, contents: &str, indent: usize) -> String {
    return render(MOD_TEMPLATE, indent, &[mod_name, contents]);
}
