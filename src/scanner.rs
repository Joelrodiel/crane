use crate::tokens::*;
use TokenType::*;

pub fn scan_tokens(source: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::<Token>::new();
    let mut index = 0;
    let mut line = 1;
    let mut chars = source.chars();
    let mut curr_ch = chars.next();
    let mut had_error = false;

    let mono_token = |typ: TokenType, lex: String, lin: u32| Token::new(typ, &lex, LiteralReturns::NIL, lin);

    while index < source.len() {
        match curr_ch {
            Some('(') => {
                tokens.push(mono_token(LEFTPAREN, curr_ch.unwrap().to_string(), line));
                curr_ch = chars.next();
            }
            Some(')') => {
                tokens.push(mono_token(RIGHTPAREN, curr_ch.unwrap().to_string(), line));
                curr_ch = chars.next();
            }
            Some('{') => {
                tokens.push(mono_token(LEFTBRACE, curr_ch.unwrap().to_string(), line));
                curr_ch = chars.next();
            }
            Some('}') => {
                tokens.push(mono_token(RIGHTBRACE, curr_ch.unwrap().to_string(), line));
                curr_ch = chars.next();
            }
            Some(',') => {
                tokens.push(mono_token(COMMA, curr_ch.unwrap().to_string(), line));
                curr_ch = chars.next();
            }
            Some('.') => {
                tokens.push(mono_token(DOT, curr_ch.unwrap().to_string(), line));
                curr_ch = chars.next();
            }
            Some('-') => {
                tokens.push(mono_token(MINUS, curr_ch.unwrap().to_string(), line));
                curr_ch = chars.next();
            }
            Some('+') => {
                tokens.push(mono_token(PLUS, curr_ch.unwrap().to_string(), line));
                curr_ch = chars.next();
            }
            Some(';') => {
                tokens.push(mono_token(SEMICOLON, curr_ch.unwrap().to_string(), line));
                curr_ch = chars.next();
            }
            Some('*') => {
                tokens.push(mono_token(STAR, curr_ch.unwrap().to_string(), line));
                curr_ch = chars.next();
            }
            Some('!') => {
                curr_ch = chars.next();
                if let Some(c) = curr_ch {
                    if c == '=' {
                        tokens.push(mono_token(BANGEQUAL, "!=".to_string(), line));
                        curr_ch = chars.next();
                        index += 1;
                    } else {
                        tokens.push(mono_token(BANG, "!".to_string(), line));
                    }
                } else {
                    tokens.push(mono_token(BANG, "!".to_string(), line));
                }
            }
            Some('=') => {
                curr_ch = chars.next();
                if let Some(c) = curr_ch {
                    if c == '=' {
                        tokens.push(mono_token(EQUALEQUAL, "==".to_string(), line));
                        curr_ch = chars.next();
                        index += 1;
                    } else {
                        tokens.push(mono_token(EQUAL, "=".to_string(), line));
                    }
                } else {
                    tokens.push(mono_token(EQUAL, "=".to_string(), line));
                }
            }
            Some('<') => {
                curr_ch = chars.next();
                if let Some(c) = curr_ch {
                    if c == '=' {
                        tokens.push(mono_token(LESSEQUAL, "<=".to_string(), line));
                        curr_ch = chars.next();
                        index += 1;
                    } else {
                        tokens.push(mono_token(LESS, "<".to_string(), line));
                    }
                } else {
                    tokens.push(mono_token(LESS, "<".to_string(), line));
                }
            }
            Some('>') => {
                curr_ch = chars.next();
                if let Some(c) = curr_ch {
                    if c == '=' {
                        tokens.push(mono_token(GREATEREQUAL, ">=".to_string(), line));
                        curr_ch = chars.next();
                        index += 1;
                    } else {
                        tokens.push(mono_token(GREATER, ">".to_string(), line));
                    }
                } else {
                    tokens.push(mono_token(GREATER, ">".to_string(), line));
                }
            }

            Some(' ') |
            Some('\r') |
            Some('\t') => {
                curr_ch = chars.next();
            }

            Some('\n') => {
                line += 1;
                curr_ch = chars.next();
            }

            Some('/') => {
                curr_ch = chars.next();
                if let Some(c) = curr_ch {
                    if c == '/' {
                        index += 1;
                        loop {
                            curr_ch = chars.next();
                            if let Some(c1) = curr_ch {
                                if c1 == '\n' || index + 2 >= source.len() { break; }
                                index += 1;
                            }
                        }
                    } else {
                        tokens.push(mono_token(SLASH, "/".to_string(), line));
                    }
                } else {
                    tokens.push(mono_token(SLASH, "/".to_string(), line));
                }
            }

            Some('"') => {
                let mut string = String::from("");
                loop {
                    curr_ch = chars.next();
                    if let Some(c) = curr_ch {
                        if c == '"' || index + 2 >= source.len() { break; }
                        string.push(c);
                        index += 1;
                    }
                }

                if index + 2 >= source.len() {
                    print_error(line, index as u32, "Unterminated string.");
                    had_error = true;
                }

                let mut lexem = String::from(&string);
                lexem.insert(0, '"');
                lexem.push('"');
                tokens.push(Token::new(STRING, &lexem, LiteralReturns::STR(string), line));

                curr_ch = chars.next();
            }

            Some(symbol) => {
                if is_digit(symbol) {
                    let mut num = String::from("");
                    num.push(symbol);
                    loop {
                        curr_ch = chars.next();
                        if let Some(c) = curr_ch {
                            if !is_digit(c) && c != '.' { break; }
                            num.push(c);
                            index += 1;
                        }
                    }

                    match num.parse::<f32>() {
                        Ok(n) => {
                            let lexem = String::from(&num);
                            tokens.push(Token::new(NUMBER, &lexem, LiteralReturns::NUM(n), line));
                        }
                        Err(_) => {
                            print_error(line, index as u32, "Invalid number syntax.");
                            had_error = true;
                        }
                    }
                } else if is_alpha(symbol) {
                    let mut word = String::from("");
                    word.push(symbol);
                    loop {
                        curr_ch = chars.next();
                        if let Some(c) = curr_ch {
                            if !is_alpha_numeric(c) { break; }
                            word.push(c);
                            index += 1;
                        }
                    }

                    let lexem = String::from(&word);
                    let key_type = get_keyword_type(&word);
                    let mut literal = LiteralReturns::NIL;
                    match key_type {
                        TRUE => {
                            literal = LiteralReturns::BOOL(true);
                        }
                        FALSE => {
                            literal = LiteralReturns::BOOL(false);
                        }
                        _ => {}
                    }

                    tokens.push(Token::new(key_type, &lexem, literal, line));
                } else {
                    print_error(line, index as u32, "Unexpected character.");
                    had_error = true;
                }
            }

            None => break
        }

        index += 1;
    }

    if had_error {
        std::process::exit(65);
    }

    tokens.push(Token::new(EOF, "", LiteralReturns::NIL, line));

    tokens
}

fn print_error(row: u32, col: u32, msg: &str) {
    println!("[line {}:col {}] Error: {}", row, col, msg);
}

fn is_digit(num: char) -> bool {
    num >= '0' && num <= '9'
}

fn is_alpha(c: char) -> bool {
    (c >= 'a' && c <= 'z') ||
    (c >= 'A' && c <= 'Z') ||
    (c == '_')
}

fn get_keyword_type(key: &str) -> TokenType {
    match key {
        "and" => AND,
        "class" => CLASS,
        "else" => ELSE,
        "false" => FALSE,
        "for" => FOR,
        "fn" => FUNC,
        "if" => IF,
        "nil" => NIL,
        "or" => OR,
        "print" => PRINT,
        "return" => RETURN,
        "super" => SUPER,
        "this" => THIS,
        "true" => TRUE,
        "var" => VAR,
        "while" => WHILE,
        _ => IDENTIFIER
    }
}

fn is_alpha_numeric(c: char) -> bool {
    is_alpha(c) || is_digit(c)
}
