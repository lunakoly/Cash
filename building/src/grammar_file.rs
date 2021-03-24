pub mod parsing;

use parsing::*;

use serde_json::Value;

use degen::rendering::{render};

const STRUCT_RULE_TEMPLATE: &'static str = "
    #[derive(Clone)]
    pub struct Branch<'a> {
        pub pattern: Vec<&'static str>,
        pub handler: &'a dyn Fn(Vec<Box<dyn crate::ast::Node>>) -> Box<dyn crate::ast::Node>
    }

    #[derive(Clone)]
    pub struct Rule<'a> {
        pub name: &'static str,
        pub simple_branches: Vec<Branch<'a>>,
        pub recursive_branches: Vec<Branch<'a>>,
    }
";

fn render_struct_rule() -> String {
    return render(STRUCT_RULE_TEMPLATE, 0, &[]);
}

const BRANCH_TEMPLATE: &'static str = "
    Branch {
        pattern: vec![$$],
        handler: &$$
    }
";

fn render_branch(branch: &Branch, indent: usize) -> String {
    let pattern = branch.pattern.iter()
        .map(|it| "\"".to_owned() + &it + "\"")
        .collect::<Vec<String>>()
        .join(", ");

    return render(BRANCH_TEMPLATE, indent, &[&pattern, &branch.handler]);
}

const RULE_TEMPLATE: &'static str = "
    Rule {
        name: $$,
        simple_branches: vec![
    $$
        ],
        recursive_branches: vec![
    $$
        ],
    }
";

fn render_rule(rule: &Rule, indent: usize) -> String {
    let rule_name = "\"".to_owned() + &rule.name + "\"";
    let mut recursive_branches = vec![];
    let mut simple_branches = vec![];

    for it in &rule.branches {
        if it.pattern[0] == "@".to_owned() + &rule.name {
            recursive_branches.push(render_branch(it, indent));
        } else {
            simple_branches.push(render_branch(it, indent));
        }
    }

    return render(
        RULE_TEMPLATE,
        indent,
        &[
            &rule_name,
            &simple_branches.join(",\n"),
            &recursive_branches.join(",\n")
        ]
    );
}

const ALL_RULES_TEMPLATE: &'static str = "
    pub fn get_rules<'a>() -> Vec<Rule<'a>> {
        return vec![
    $$
        ];
    }
";

fn render_all_rules(grammar_file: &GrammarFile) -> String {
    let mut rules = vec![];

    for it in &grammar_file.rules {
        rules.push(render_rule(it, 8));
    }

    return render(ALL_RULES_TEMPLATE, 0, &[&rules.join(",\n")]);
}

/// Parses the template as the grammar.json file
/// and renders the corresponding rust source code.
pub fn ast_to_source(template: Value) -> String {
    let grammar_file = parse_ast(template);

    let pieces = vec![
        "// THIS CODE IS AUTO-GENERATED".to_owned(),
        render_struct_rule(),
        render_all_rules(&grammar_file)
    ];

    return pieces.join("\n\n");
}
