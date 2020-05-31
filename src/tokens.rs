pub enum TokenType {
    LEFTPAREN, RIGHTPAREN, LEFTBRACE, RIGHTBRACE,
    COMMA, DOT, MINUS, PLUS, SEMICOLON, SLASH, STAR,

    BANG, BANGEQUAL,
    EQUAL, EQUALEQUAL,
    GREATER, GREATEREQUAL,
    LESS, LESSEQUAL,

    IDENTIFIER, STRING, NUMBER,

    AND, CLASS, ELSE, FALSE, FUNC, FOR, IF, NIL, OR,
    PRINT, RETURN, SUPER, THIS, TRUE, VAR, WHILE,

    EOF
}

pub enum LiteralReturns {
    BOOL(bool),
    NUM(f32),
    STR(String),
    NIL,
}

pub struct Token {
    typ: TokenType,
    lexeme: String,
    literal: LiteralReturns,
    line: u32,
}

impl Token {
    pub fn new(typ: TokenType, lexeme: &str, literal: LiteralReturns, line: u32) -> Token {
        Token {
            typ: typ,
            lexeme: lexeme.to_string(),
            literal: literal,
            line: line,
        }
    }
    pub fn print(&self) {
        match &self.literal {
            LiteralReturns::STR(string) => {
                println!("Token: {}, String: {}, line: {}", self.lexeme, string, self.line);
            }
            LiteralReturns::NUM(num) => {
                println!("Token: {}, Number: {}, line: {}", self.lexeme, num, self.line);
            }
            LiteralReturns::BOOL(state) => {
                println!("Token: {}, Bool: {}, line: {}", self.lexeme, state, self.line);
            }
            _ => {
                match self.typ {
                    TokenType::EOF => {
                        println!("Token: EOF, line: {}", self.line);
                    }
                    _ => {
                        println!("Token: {}, line: {}", self.lexeme, self.line);
                    }
                }
            }
        }
    }
}
