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

use crate::stream::TokenStream;

pub enum ArithmeticExpr {
    Addition(Box<ArithmeticExpr>, Box<ArithmeticExpr>),
    Subtraction(Box<ArithmeticExpr>, Box<ArithmeticExpr>),
    Multiplication(Box<ArithmeticExpr>, Box<ArithmeticExpr>),
    Division(Box<ArithmeticExpr>, Box<ArithmeticExpr>),
    Modulus(Box<ArithmeticExpr>, Box<ArithmeticExpr>),

    Float(f64),
    Integer(u128)
}



impl TokenStream {

    pub(crate) fn try_parse_arithmetic_expr(&mut self) -> Option<ArithmeticExpr> {
        None
    }

}