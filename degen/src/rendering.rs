/// Calculates the number of spaces (or 4 * \t)
/// between the first non-whitespace character
/// and the preceding \n or the string start.
pub fn calculate_indent(template: &str) -> usize {
    let mut indent: usize = 0;

    for it in template.chars() {
        if it == ' ' {
            indent += 1;
        } else if it == '\t' {
            indent += 4;
        } else if it == '\n' {
            indent = 0;
        } else {
            break
        }
    }

    return indent;
}

/// Returns the template-based string
/// with the arguments substituted in place
/// of placeholders.
///
/// If the first/last character is \n, it is
/// removed.
pub fn render(
    template: &str,
    indent: usize,
    arguments: &[&str]
) -> String {
    let old_indent = calculate_indent(&template);
    let old_shift = "\n".to_owned() + &" ".repeat(old_indent);
    let new_shift = "\n".to_owned() + &" ".repeat(indent);
    let mut result = template.replace(&old_shift, &new_shift);

    if result.chars().next() == Some('\n') {
        result.remove(0);
    }

    if result.chars().last() == Some('\n') {
        result.pop();
    }

    for it in arguments {
        result = result.replacen("$$", it, 1);
    }

    return result;
}

/// If argument is not empty, returns
/// the rendered template with this single
/// argument. Otherwise, returns an empty string.
pub fn render_non_empty(
    template: &str,
    argument: &str
) -> String {
    if argument.is_empty() {
        String::new()
    } else {
        render(template, 0, &[argument])
    }
}
