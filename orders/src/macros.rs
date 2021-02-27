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
