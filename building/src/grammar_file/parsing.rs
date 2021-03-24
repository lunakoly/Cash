use serde_json::Value;

pub struct Branch {
    pub pattern: Vec<String>,
    pub handler: String,
}

pub struct Rule {
    pub name: String,
    pub branches: Vec<Branch>,
}

pub struct GrammarFile {
    pub rules: Vec<Rule>,
}

fn parse_branches(rule_body_json: &Value) -> Vec<Branch> {
    let mut branches = vec!();

    for (pattern_string, handler) in rule_body_json.as_object().unwrap() {
        let branch = Branch {
            pattern: pattern_string.split(" ").map(|it| it.to_owned()).collect(),
            handler: handler.as_str().unwrap().to_owned(),
        };

        branches.push(branch);
    }

    return branches;
}

fn parse_rules(rules_json: &Value) -> Vec<Rule> {
    let mut rules = vec!();

    for (name, rule_body) in rules_json.as_object().unwrap() {
        let rule = Rule {
            name: name.to_owned(),
            branches: parse_branches(rule_body)
        };

        rules.push(rule);
    }

    return rules;
}

pub fn parse_ast(template: Value) -> GrammarFile {
    GrammarFile {
        rules: parse_rules(&template["rules"]),
    }
}
