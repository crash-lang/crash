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

pub enum ExpressionDecl {
    Call { name: String },

    // Math
    Add { summand1: Box<ExpressionDecl>, summand2: Box<ExpressionDecl> },
    Subtract { subtrahend1: Box<ExpressionDecl>, subtrahend2: Box<ExpressionDecl> },
    Multiply { factor1: Box<ExpressionDecl>, factor2: Box<ExpressionDecl> },
    Divide { divisor1: Box<ExpressionDecl>, divisor2: Box<ExpressionDecl> },
    Modulus { modulo1: Box<ExpressionDecl>, modulo2: Box<ExpressionDecl> },

    // Logic
    Equals { first: Box<ExpressionDecl>, second: Box<ExpressionDecl> },
    NotEquals { first: Box<ExpressionDecl>, second: Box<ExpressionDecl> },
    Greater { first: Box<ExpressionDecl>, second: Box<ExpressionDecl> },
    GreaterEquals { first: Box<ExpressionDecl>, second: Box<ExpressionDecl> },
    Less { first: Box<ExpressionDecl>, second: Box<ExpressionDecl> },
    LessEquals { first: Box<ExpressionDecl>, second: Box<ExpressionDecl> },
    And { first: Box<ExpressionDecl>, second: Box<ExpressionDecl> },
    Or { first: Box<ExpressionDecl>, second: Box<ExpressionDecl> },
    Not { expr: Box<ExpressionDecl> },

    /// Simply self keyword
    LocalSelf,
    /// Simply this keyword
    LocalThis,

    Bool { val: bool },
    //TODO Other literals
}