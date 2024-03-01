
#[macro_export]
macro_rules! tok_struct {
    (
        $struct_name:ident $(< $($lt:lifetime), +>)? 
        {
            $($field_name:ident : $field_type:ty), * $(,)?
        }
    ) => {
        use crate::structure::*;
        use crash_ir_lexer::{Token, TokenPosition};

        pub struct $struct_name $(<$($lt), + >)? {
            tokens: Vec<Token>,
            position: TokenPosition,
            $($field_name : $field_type), *
        }

        impl $(<$($lt), + >)? StructureInfo for $struct_name $(<$($lt), + >)? {

            fn tokens(self) -> Vec<Token> {
                self.tokens
            }

            fn position(&self) -> TokenPosition {
                self.position
            }
        }
    };
}