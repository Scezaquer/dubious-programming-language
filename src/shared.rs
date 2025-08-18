/// Monad to attach types to elements of the AST

/// A token with extra debug information to print more useful error messages.
#[derive(Debug, Clone)]
pub struct TokenWithDebugInfo<T> {
    pub internal_tok: T,
    pub line: usize,
    pub file: String,
}

impl<T: PartialEq> PartialEq for TokenWithDebugInfo<T> {
    fn eq(&self, other: &Self) -> bool {
        self.internal_tok == other.internal_tok
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Int,
    Float,
    Void,
    Char,
    Bool,
    Pointer(Box<TokenWithDebugInfo<Type>>),
    Array(Box<TokenWithDebugInfo<Type>>), // array[type]. Strings are array[char]
    Struct(String),
    Enum(String),
    Namespace(String, Box<TokenWithDebugInfo<Type>>),
    GenericBinding(String, Vec<TokenWithDebugInfo<Type>>),
}

#[derive(Debug, Clone)]
pub struct Typed<T> {
    pub expr: T,
    pub type_: Type,
}

impl<T> Typed<T> {
    /// Creates a new Typed instance with a void type.
    pub fn new(expr: T) -> Self {
        Typed {
            expr,
            type_: Type::Void,
        }
    }

    pub fn new_with_type(expr: T, type_: Type) -> Self {
        Typed { expr, type_ }
    }

    pub fn get_type(&self) -> &Type {
        &self.type_
    }
}

pub fn error<T>(msg: &str, token: &TokenWithDebugInfo<T>) -> ! {
    panic!("{}:{}: {}", token.file, token.line, msg);
}

pub fn error_unexpected_token<T>(expected: &str, token: &TokenWithDebugInfo<T>) -> !
where
    T: std::fmt::Debug,
{
    error(
        &format!("Expected {}, found: {:?}", expected, token.internal_tok),
        token,
    );
}
