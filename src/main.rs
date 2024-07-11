use astray_core::{matcher, print_error, Pattern, Token};

fn main() {
    Pattern {
    fun:|t|{
        matches!(t,Token::INVALID)
    },pat:"Token :: INVALID"
};
}
