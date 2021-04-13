// @generated by xtask generate-grammar
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[repr(u16)]
pub enum SyntaxKind {
    ERROR,
    EOF,
    TOMBSTONE,
    PLUS_EQ,
    MINUS_EQ,
    MUL_EQ,
    DIV_EQ,
    MOD_EQ,
    PLUS,
    MINUS,
    ASTERISK,
    SLASH,
    PERCENT,
    EQ_EQ,
    GREATER_EQ,
    GREATER,
    LESS_EQ,
    LESS,
    EQ,
    NEQ,
    L_PAREN,
    R_PAREN,
    L_BRACKET,
    R_BRACKET,
    L_BRACE,
    R_BRACE,
    DOT,
    COLON,
    QMARK,
    SEMICOLON,
    COMMA,
    AND_KW,
    OR_KW,
    NOT_KW,
    IN_KW,
    LET_KW,
    IF_KW,
    ELSE_KW,
    FOREACH_KW,
    CONTINUE_KW,
    BREAK_KW,
    RETURN_KW,
    TRUE_KW,
    FALSE_KW,
    FN_KW,
    STRUCT_KW,
    NEWLINE,
    ID,
    NUM,
    STRING,
    MULTILINE_STRING,
    SINGLE_LINE_COMMENT,
    BLOCK_COMMENT,
    WHITESPACE,
    Expr,
    ExprBlock,
    KExpr,
    ArrayLitExpr,
    MapLitExpr,
    StrLit,
    PrimaryExpr,
    PrefixUnaryOpExpr,
    InfixBinOpExpr,
    FnCallExpr,
    FnCallArgsList,
    FArg,
    TupleExpr,
    BoolLit,
    IndexExpr,
    IndexExprBrackets,
    Assignment,
    Declaration,
    Conditional,
    Foreach,
    ForInExpr,
    ControlStatement,
    ExprStatement,
    Tuple,
    ConditionalBranch,
    Statement,
    StructDecl,
    StructFieldList,
    StructField,
    TypeRef,
    TypeRefGenerics,
    LangItem,
    ROOT,
}
impl From<SyntaxKind> for rowan::SyntaxKind {
    fn from(kind: SyntaxKind) -> Self {
        Self(kind.into())
    }
}
impl From<&SyntaxKind> for rowan::SyntaxKind {
    fn from(kind: &SyntaxKind) -> Self {
        Self(kind.into())
    }
}
#[allow(clippy::fallible_impl_from)]
impl From<u16> for SyntaxKind {
    fn from(i: u16) -> Self {
        assert!(i <= Self::ROOT as u16);
        #[allow(unsafe_code)]
        unsafe {
            std::mem::transmute::<u16, Self>(i)
        }
    }
}
impl From<SyntaxKind> for u16 {
    fn from(kind: SyntaxKind) -> Self {
        kind as Self
    }
}
impl From<&SyntaxKind> for u16 {
    fn from(kind: &SyntaxKind) -> Self {
        (*kind).into()
    }
}
impl SyntaxKind {
    pub(crate) fn token_name(&self) -> &'static str {
        use SyntaxKind::*;
        match self {
            PLUS_EQ => "+=",
            MINUS_EQ => "-=",
            MUL_EQ => "*=",
            DIV_EQ => "/=",
            MOD_EQ => "%=",
            PLUS => "+",
            MINUS => "-",
            ASTERISK => "*",
            SLASH => "/",
            PERCENT => "%",
            EQ_EQ => "==",
            GREATER_EQ => ">=",
            GREATER => ">",
            LESS_EQ => "<=",
            LESS => "<",
            EQ => "=",
            NEQ => "!=",
            L_PAREN => "(",
            R_PAREN => ")",
            L_BRACKET => "[",
            R_BRACKET => "]",
            L_BRACE => "{",
            R_BRACE => "}",
            DOT => ".",
            COLON => ":",
            QMARK => "?",
            SEMICOLON => ";",
            COMMA => ",",
            AND_KW => "and",
            OR_KW => "or",
            NOT_KW => "not",
            IN_KW => "in",
            LET_KW => "let",
            IF_KW => "if",
            ELSE_KW => "else",
            FOREACH_KW => "foreach",
            CONTINUE_KW => "continue",
            BREAK_KW => "break",
            RETURN_KW => "return",
            TRUE_KW => "true",
            FALSE_KW => "false",
            FN_KW => "fn",
            STRUCT_KW => "struct",
            NEWLINE => "\n",
            ID => "Id",
            NUM => "NumLit",
            STRING => "Str",
            MULTILINE_STRING => "MultilineStr",
            SINGLE_LINE_COMMENT => "SingleLineComment",
            BLOCK_COMMENT => "BlockComment",
            WHITESPACE => "Whitespace",
            _ => "",
        }
    }
}
#[macro_export]
macro_rules ! T { [+=] => { $ crate :: SyntaxKind :: PLUS_EQ } ; [-=] => { $ crate :: SyntaxKind :: MINUS_EQ } ; [*=] => { $ crate :: SyntaxKind :: MUL_EQ } ; [/=] => { $ crate :: SyntaxKind :: DIV_EQ } ; [%=] => { $ crate :: SyntaxKind :: MOD_EQ } ; [+] => { $ crate :: SyntaxKind :: PLUS } ; [-] => { $ crate :: SyntaxKind :: MINUS } ; [*] => { $ crate :: SyntaxKind :: ASTERISK } ; [/] => { $ crate :: SyntaxKind :: SLASH } ; [%] => { $ crate :: SyntaxKind :: PERCENT } ; [==] => { $ crate :: SyntaxKind :: EQ_EQ } ; [>=] => { $ crate :: SyntaxKind :: GREATER_EQ } ; [>] => { $ crate :: SyntaxKind :: GREATER } ; [<=] => { $ crate :: SyntaxKind :: LESS_EQ } ; [<] => { $ crate :: SyntaxKind :: LESS } ; [=] => { $ crate :: SyntaxKind :: EQ } ; [!=] => { $ crate :: SyntaxKind :: NEQ } ; ['('] => { $ crate :: SyntaxKind :: L_PAREN } ; [')'] => { $ crate :: SyntaxKind :: R_PAREN } ; ['['] => { $ crate :: SyntaxKind :: L_BRACKET } ; [']'] => { $ crate :: SyntaxKind :: R_BRACKET } ; ['{'] => { $ crate :: SyntaxKind :: L_BRACE } ; ['}'] => { $ crate :: SyntaxKind :: R_BRACE } ; [.] => { $ crate :: SyntaxKind :: DOT } ; [:] => { $ crate :: SyntaxKind :: COLON } ; [?] => { $ crate :: SyntaxKind :: QMARK } ; [;] => { $ crate :: SyntaxKind :: SEMICOLON } ; [,] => { $ crate :: SyntaxKind :: COMMA } ; [and] => { $ crate :: SyntaxKind :: AND_KW } ; [or] => { $ crate :: SyntaxKind :: OR_KW } ; [not] => { $ crate :: SyntaxKind :: NOT_KW } ; [in] => { $ crate :: SyntaxKind :: IN_KW } ; [let] => { $ crate :: SyntaxKind :: LET_KW } ; [if] => { $ crate :: SyntaxKind :: IF_KW } ; [else] => { $ crate :: SyntaxKind :: ELSE_KW } ; [foreach] => { $ crate :: SyntaxKind :: FOREACH_KW } ; [continue] => { $ crate :: SyntaxKind :: CONTINUE_KW } ; [break] => { $ crate :: SyntaxKind :: BREAK_KW } ; [return] => { $ crate :: SyntaxKind :: RETURN_KW } ; [true] => { $ crate :: SyntaxKind :: TRUE_KW } ; [false] => { $ crate :: SyntaxKind :: FALSE_KW } ; [fn] => { $ crate :: SyntaxKind :: FN_KW } ; [struct] => { $ crate :: SyntaxKind :: STRUCT_KW } ; ['\n'] => { $ crate :: SyntaxKind :: NEWLINE } ; [Id] => { $ crate :: SyntaxKind :: ID } ; [NumLit] => { $ crate :: SyntaxKind :: NUM } ; [Str] => { $ crate :: SyntaxKind :: STRING } ; [MultilineStr] => { $ crate :: SyntaxKind :: MULTILINE_STRING } ; [SingleLineComment] => { $ crate :: SyntaxKind :: SINGLE_LINE_COMMENT } ; [BlockComment] => { $ crate :: SyntaxKind :: BLOCK_COMMENT } ; [Whitespace] => { $ crate :: SyntaxKind :: WHITESPACE } ; }
