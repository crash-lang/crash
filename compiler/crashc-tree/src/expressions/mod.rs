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

pub mod math;
pub mod literal;
pub mod assign;
pub mod logic;
pub mod call;

use crashc_lexer::position::TokenPosition;
use crate::expressions::assign::Assign;
use crate::expressions::call::FunctionCall;
use crate::expressions::literal::Literal;
use crate::expressions::logic::*;
use crate::expressions::math::*;

#[derive(Clone, Debug)]
pub enum ExpressionType {
    Literal(Literal),

    Addition(Addition),
    Subtraction(Subtraction),
    Multiplication(Multiplication),
    Division(Division),
    Modulus(Modulus),

    Assign(Assign),

    And(And),
    Or(Or),
    Not(Not),
    Equals(Equals),
    Less(Less),
    Greater(Greater),
    LessOrEqual(LessOrEqual),
    GreaterOrEqual(GreaterOrEqual),

    FunctionCall(FunctionCall)
}

pub trait Expression {

    fn expression_type(self) -> ExpressionType;

    fn position(&self) -> TokenPosition;
}

#[macro_export]
macro_rules! define_expression {
    ($name:ident) => {
        impl Expression for $name {
            fn expression_type(self) -> ExpressionType {
                ExpressionType::$name(self)
            }

            fn position(&self) -> TokenPosition {
                self.position
            }
        }
    };
}