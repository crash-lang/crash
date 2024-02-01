/*
 * Copyright 2024 Julian Siebert
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 */

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