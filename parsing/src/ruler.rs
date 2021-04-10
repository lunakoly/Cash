use helpers::{elvis, some_or};

/// A Token that's value can be turned
/// into a string, and that has a
/// string-representable type.
pub trait RepresentableToken {
    fn get_type_name(&self) -> String;
    fn get_value(&self) -> Option<&str>;
}

/// A single branch of a rule with
/// a pattern and a handler called if
/// that pattern is met.
pub struct Branch<'a, A> {
    pub pattern: Vec<&'static str>,
    pub handler: &'a dyn Fn(Vec<A>) -> A
}

/// A rule for an entity. It may
/// have multiple branches, and some
/// may be left-recurrent (`recursive_branches`).
pub struct Rule<'a, A> {
    pub name: &'static str,
    pub simple_branches: Vec<Branch<'a, A>>,
    pub recursive_branches: Vec<Branch<'a, A>>,
}

/// A collection of rules for all entities
/// along with a function for turning tokens
/// into the corresponding node.
pub struct Grammar<'a, A, T: RepresentableToken> {
    pub handle_token: &'a dyn Fn(&T) -> A,
    pub rules: Vec<Rule<'a, A>>,
}

/// Returns a reference to
/// a rule for the given entity or
/// None if there's no such a rule.
fn get_rule_by_name<'a, A>(
    rules: &'a Vec<Rule<'a, A>>, name: &str
) -> Option<&'a Rule<'a, A>> {
    for rule in rules {
        if rule.name == name {
            return Some(rule);
        }
    }

    return None;
}

/// Checks the next token
/// against the proper rule.
fn apply_item<A, T: RepresentableToken>(
    item: &str,
    tokens: &[T],
    token_index: usize,
    grammar: &Grammar<A, T>,
) -> (Option<A>, usize) {
    if token_index >= tokens.len() {
        return (None, token_index);
    }

    if item.len() > 1 && item.starts_with("@") {
        let rule_name = item.chars().skip(1).collect::<String>();
        return apply_rule(&rule_name, tokens, token_index, grammar);
    }

    if item.starts_with("#") {
        let token_type = item.chars().skip(1).collect::<String>();

        if tokens[token_index].get_type_name() != token_type {
            return (None, token_index);
        }

        return (
            Some((grammar.handle_token)(&tokens[token_index])),
            token_index + 1
        );
    }

    if Some(item) == tokens[token_index].get_value() {
        return (
            Some((grammar.handle_token)(&tokens[token_index])),
            token_index + 1
        );
    }

    return (None, token_index);
}

/// Checks the next token
/// agains the specified branch.
fn apply_branch<A, T: RepresentableToken>(
    branch: &Branch<A>,
    pattern_item_index: usize,
    tokens: &[T],
    token_index: usize,
    grammar: &Grammar<A, T>,
) -> (Option<Vec<A>>, usize) {
    let mut moved_token_index = token_index;
    let mut values = vec![];

    for it in pattern_item_index..branch.pattern.len() {
        let (item, new_token_index) = apply_item(
            branch.pattern[it],
            tokens,
            moved_token_index,
            grammar,
        );

        if let Some(thing) = item {
            values.push(thing);
            moved_token_index = new_token_index;
        } else {
            return (None, token_index);
        }
    }

    return (Some(values), moved_token_index);
}

/// Checks the next token agains
/// a simple rule (non left-recurrent).
fn apply_simple_rule<A, T: RepresentableToken>(
    rule_name: &str,
    tokens: &[T],
    token_index: usize,
    grammar: &Grammar<A, T>,
) -> (Option<A>, usize) {
    let rule = some_or! {
        get_rule_by_name(&grammar.rules, rule_name) =>
        return (None, token_index)
    };

    for branch in &rule.simple_branches {
        let (values, new_token_index) = apply_branch(
            branch,
            0,
            tokens,
            token_index,
            grammar,
        );

        if let Some(values) = values {
            return (Some((branch.handler)(values)), new_token_index);
        }
    }

    return (None, token_index);
}

/// Checks the next token agains
/// the specified rule. First cheks 'simple'
/// rules, then - attempts to apply left-recurrent
/// rules in a loop.
pub fn apply_rule<A, T: RepresentableToken>(
    rule_name: &str,
    tokens: &[T],
    token_index: usize,
    grammar: &Grammar<A, T>,
) -> (Option<A>, usize) {
    let (simple_result, mut moved_token_index) = apply_simple_rule(
        rule_name,
        tokens,
        token_index,
        grammar,
    );

    let mut result = some_or! {
        simple_result => return (None, token_index)
    };

    let rule = some_or! {
        get_rule_by_name(&grammar.rules, rule_name) =>
        return (None, token_index)
    };

    let mut applied = true;

    while applied {
        applied = false;

        for branch in &rule.recursive_branches {
            let (maybe_values, new_token_index) = apply_branch(
                branch,
                1,
                tokens,
                moved_token_index,
                grammar,
            );

            if let Some(mut values) = maybe_values {
                values.insert(0, result);
                result = (branch.handler)(values);
                moved_token_index = new_token_index;
                applied = true;
                break;
            }
        }
    }

    return (Some(result), moved_token_index);
}
