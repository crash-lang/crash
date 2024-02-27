#[macro_export]
macro_rules! find_rule {
    ( $rules:expr, $typ:expr ) => {{
        let mut the_rule = LexingRule::new(Vec::new(), $typ);

        for rule in $rules.clone() {
            if rule.typ() == $typ {
                the_rule = rule;
            }
        }

        the_rule
    }};
}

#[macro_export]
macro_rules! add_rule {
    ( $rules:expr, $typ:expr , $content:expr) => {
        {
            let mut rule = find_rule!($rules, $typ);
            rule.add($content.to_string());
            $rules.push(rule);
        }
    };
}

#[macro_export]
macro_rules! add_regex_rule {
    ( $rules:expr, $typ:expr , $content:expr) => {
        {
            let mut rule = find_rule!($rules, $typ);
            rule.add_regex($content.to_string());
            $rules.push(rule);
        }
    };
}

#[macro_export]
macro_rules! add_multi_line_rule {
    ( $rules:expr, $typ:expr , $open:expr, $close:expr) => {
        {
            let mut rule = find_rule!($rules, $typ);
            rule.add_multi_line($open.to_string(), $close.to_string());
            $rules.push(rule);
        }
    };
}