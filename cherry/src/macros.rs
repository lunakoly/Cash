#[macro_export]
macro_rules! within {
    ( $this:ident, $opening:expr, $closing:expr => $($action:expr);* ) => {
        $this.skip_blank();

        if $this.input.accept($opening) {
            let it = {
                $($action);*
            };
            $this.input.accept($closing);
            return it;
        }
    }
}

#[macro_export]
macro_rules! within_parentheses {
    ( $this:ident => $($action:expr);* ) => {
        within! { $this, '(', ')' =>
            $($action);*
        };
    }
}

#[macro_export]
macro_rules! parse_binary {
    ( $this:expr, $inner:ident => $condition:expr ) => {
        let mut it = $this.$inner();

        while $condition {
            it = Box::new(Binary {
                lefter: it,
                operator: Box::new(Leaf { value: $this.input.revise_all() }),
                righter: $this.$inner(),
            });
        }

        return it;
    };
}

#[macro_export]
macro_rules! parse_list {
    ( $this:expr, $inner:ident, $closing:expr) => {
        let mut it = vec![$this.$inner()];

        while $this.expect_operator(",") {
            $this.skip_blank();

            if $this.input.match_text($closing) == $closing.len() {
                break;
            }

            it.push($this.$inner());
        }

        return it;
    };
}
