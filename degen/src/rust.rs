pub mod visitors;

use crate::rendering::*;

/// A single field of a struct.
pub struct FieldInfo {
    pub name: String,
    pub proto: String,
}

/// Template for rendering
/// individual struct fields.
const FIELD_TEMPLATE: &'static str = "
    pub $$: $$,
";

/// Returns the string representation
/// of a struct field.
pub fn render_field(field: &FieldInfo, indent: usize) -> String {
    return render(FIELD_TEMPLATE, indent, &[&field.name, &field.proto]);
}

/// Template for rendering structs.
const STRUCT_TEMPLATE: &'static str = "
    pub struct $$ {
    $$
    }
";

/// Returns the struct representation.
pub fn render_struct(name: &str, fields: &[FieldInfo], indent: usize) -> String {
    let mut pieces = vec!();

    for it in fields {
        pieces.push(render_field(it, 4));
    }

    return render(STRUCT_TEMPLATE, indent, &[name, &pieces.join("\n")]);
}

/// Template for a struct declaration
/// without a body with inner fields.
const STRUCT_DECLARATION_NO_BODY: &'static str = "
    pub struct $$;
";

/// Returns the struct representation without the body.
pub fn render_struct_no_body(name: &str, indent: usize) -> String {
    return render(STRUCT_DECLARATION_NO_BODY, indent, &[name]);
}

/// template for a trait declaration.
const TRAIT_TEMPLATE: &'static str = "
    pub trait $$ {
    $$
    }
";

/// Returns the trait representation.
pub fn render_trait(name: &str, methods: &str, indent: usize) -> String {
    return render(TRAIT_TEMPLATE, indent, &[name, methods]);
}

/// Template for declaring an impl
/// for something.
const IMPL_TEMPLATE: &'static str = "
    impl $$ for $$ {
    $$
    }
";

/// Returns the impl representation.
pub fn render_impl(trait_name: &str, struct_name: &str, methods: &str, indent: usize) -> String {
    return render(IMPL_TEMPLATE, indent, &[trait_name, struct_name, methods]);
}

/// Template for declaring modules.
const MOD_TEMPLATE: &'static str = "
    pub mod $$ {
    $$
    }
";

/// Returns the impl representation.
pub fn render_mod(mod_name: &str, contents: &str, indent: usize) -> String {
    return render(MOD_TEMPLATE, indent, &[mod_name, contents]);
}

/// Template for the `#[derive(SomeTrait)]`
/// declarations.
const DERIVE_TEMPLATE: &'static str = "
    #[derive($$)]
";

/// Returns the #[derive(...)] statement
pub fn render_derive(traits: &[&str], indent: usize) -> String {
    return render(DERIVE_TEMPLATE, indent, &[&traits.join(", ")]);
}
