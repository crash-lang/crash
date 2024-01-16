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
macro_rules! syntax_error {
    ( $pos:expr, $input:expr, $message:expr, $suggestion:expr) => {
        let pos = get_pos($pos, $input.to_string());
        let row = pos.0;
        let column = pos.1;

        println!("{} at {}:{} [{}]", $message.to_string(), row, column, $pos);
        println!("{}", $suggestion);

        std::process::exit(1);
    };
}

#[macro_export]
macro_rules! keyword {
    ( $self:expr, $literal:expr , $kind:expr) => {
        if let Some(token) = $self.try_match_keyword($literal, $kind) {
            return Some(token);
        }
    };
}


#[macro_export]
macro_rules! unexpected_eof {
    ($pos:expr, $input:expr) => {
        syntax_error!($pos, $input, "Unexpected end of file", "")
    };
}

#[macro_export]
macro_rules! lock_cursor {
    ( $tokenizer:expr ) => {
        match $tokenizer.cursor.lock() {
            Ok(cursor) => {
                cursor
            }
            Err(err) => {
                panic!("Unable to lock cursor: {:?}", err)
            }
        }
    };
}