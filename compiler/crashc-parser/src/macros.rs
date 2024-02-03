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
macro_rules! check_eof {
    ($stream:expr) => {
        if !$stream.has_more_tokens() {
            unexpected_eof!($stream);
        }
    };
}

#[macro_export]
macro_rules! unexpected_eof {
    ($stream:expr) => {
        unexpected_eof_at!($stream.module_name(), $stream.current_pos());
    };
}

#[macro_export]
macro_rules! unexpected_eof_at {
    ($module_name:expr, $position:expr) => {
        println!("Unexpected end of file in module {} [{:?}]", $module_name, $position);
        std::process::exit(1);
    };
}

#[macro_export]
macro_rules! syntax_error {
    ($message:expr, $module_name:expr, $position:expr) => {
        println!("{} in module {} [{:?}]", $message, $module_name, $position);
        std::process::exit(1);
    };
}
