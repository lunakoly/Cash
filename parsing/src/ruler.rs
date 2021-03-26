pub trait RepresentableToken {
    fn get_type_name(&self) -> String;
    fn get_value(&self) -> Option<&str>;
}

pub struct Branch<'a, A> {
    pub pattern: Vec<&'static str>,
    pub handler: &'a dyn Fn(Vec<A>) -> A
}

pub struct Rule<'a, A> {
    pub name: &'static str,
    pub simple_branches: Vec<Branch<'a, A>>,
    pub recursive_branches: Vec<Branch<'a, A>>,
}

pub struct Grammar<'a, A, T: RepresentableToken> {
    pub handle_token: &'a dyn Fn(&T) -> A,
    pub rules: Vec<Rule<'a, A>>,
}

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

        if tokens[token_index].get_type_name() == token_type {
            return (
                Some((grammar.handle_token)(&tokens[token_index])),
                token_index + 1
            );
        }

        return (None, token_index);
    }

    if Some(item) == tokens[token_index].get_value() {
        return (
            Some((grammar.handle_token)(&tokens[token_index])),
            token_index + 1
        );
    }

    return (None, token_index);
}

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

fn apply_simple_rule<A, T: RepresentableToken>(
    rule_name: &str,
    tokens: &[T],
    token_index: usize,
    grammar: &Grammar<A, T>,
) -> (Option<A>, usize) {
    if let Some(rule) = get_rule_by_name(&grammar.rules, rule_name) {
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
    }

    return (None, token_index);
}

pub fn apply_rule<A, T: RepresentableToken>(
    rule_name: &str,
    tokens: &[T],
    token_index: usize,
    grammar: &Grammar<A, T>,
) -> (Option<A>, usize) {
    let (mut result, mut moved_token_index) = apply_simple_rule(
        rule_name,
        tokens,
        token_index,
        grammar,
    );

    if let Some(rule) = get_rule_by_name(&grammar.rules, rule_name) {
        let mut applied = true;

        while applied {
            applied = false;

            if let Some(mut thing) = result {
                for branch in &rule.recursive_branches {
                    let (maybe_values, new_token_index) = apply_branch(
                        branch,
                        1,
                        tokens,
                        moved_token_index,
                        grammar,
                    );

                    if let Some(mut values) = maybe_values {
                        values.insert(0, thing);
                        thing = (branch.handler)(values);
                        moved_token_index = new_token_index;
                        applied = true;
                        break;
                    }
                }

                result = Some(thing);
            }
        }

        return (result, moved_token_index);
    }

    return (None, token_index);
}
