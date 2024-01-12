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

pub enum Expression {
    Call { name: String },

    // Math
    Add { summand1: Box<Expression>, summand2: Box<Expression> },
    Subtract { subtrahend1: Box<Expression>, subtrahend2: Box<Expression> },
    Multiply { factor1: Box<Expression>, factor2: Box<Expression> },
    Divide { divisor1: Box<Expression>, divisor2: Box<Expression> },
    Modulus { modulo1: Box<Expression>, modulo2: Box<Expression> },

    // Logic
    Equals { first: Box<Expression>, second: Box<Expression> },
    NotEquals { first: Box<Expression>, second: Box<Expression> },
    Greater { first: Box<Expression>, second: Box<Expression> },
    GreaterEquals { first: Box<Expression>, second: Box<Expression> },
    Less { first: Box<Expression>, second: Box<Expression> },
    LessEquals { first: Box<Expression>, second: Box<Expression> },
    And { first: Box<Expression>, second: Box<Expression> },
    Or { first: Box<Expression>, second: Box<Expression> },
    Not { expr: Box<Expression> },

    /// Simply self keyword
    LocalSelf,
    /// Simply this keyword
    LocalThis,

    Bool { val: bool },
    //TODO Other literals
}