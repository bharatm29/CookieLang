use std::{any::Any, collections::HashMap, fmt::Display };

trait Substr {
    fn substr(&self, start: usize, end: usize) -> String;
}

impl Substr for String {
    fn substr(&self, start: usize, end: usize) -> String {
        self.to_string().chars().skip(start).take(end - start).collect()
    }
}

#[derive(Debug, Clone)]
pub enum TokenType {
    LEFTPAREN, RIGHTPAREN, LEFTBRACE, RIGHTBRACE,
    COMMA, DOT, MINUS, PLUS, SEMICOLON, SLASH, STAR,

    // One or two character tokens.
    BANG, BANGEQUAL,
    EQUAL, EQUALEQUAL,
    GREATER, GREATEREQUAL,
    LESS, LESSEQUAL,

    // Literals.
    IDENTIFIER, STRING, NUMBER,

    // Keywords.
    AND, CLASS, ELSE, FALSE, FUN, FOR, IF, NIL, OR,
    PRINT, RETURN, SUPER, THIS, TRUE, VAR, WHILE,

    EOF
}

#[derive(Debug)]
pub struct Token {
    literal: Box<dyn Any>,
    lexeme: String,
    line: usize,
    token_type: TokenType
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Token Type: {:?},\nLine: {},\nLexeme: {}\nLiteral: {:?}\n", self.token_type, self.line, self.lexeme, self.literal)
    }
}

pub struct Scanner{
    source: String,
    tokens: Vec<Token>,
    keywords: HashMap<String, TokenType>,
    start: usize,
    current: usize,
    line: usize
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 0,
            keywords: Scanner::init_keywords()
        }
    }

    fn init_keywords() -> HashMap<String, TokenType> {
        let mut keywords = HashMap::new();

        keywords.insert(String::from("and"),    TokenType::AND);
        keywords.insert(String::from("class"),  TokenType::CLASS);
        keywords.insert(String::from("else"),   TokenType::ELSE);
        keywords.insert(String::from("false"),  TokenType::FALSE);
        keywords.insert(String::from("for"),    TokenType::FOR);
        keywords.insert(String::from("fun"),    TokenType::FUN);
        keywords.insert(String::from("if"),     TokenType::IF);
        keywords.insert(String::from("nil"),    TokenType::NIL);
        keywords.insert(String::from("or"),     TokenType::OR);
        keywords.insert(String::from("print"),  TokenType::PRINT);
        keywords.insert(String::from("return"), TokenType::RETURN);
        keywords.insert(String::from("super"),  TokenType::SUPER);
        keywords.insert(String::from("this"),   TokenType::THIS);
        keywords.insert(String::from("true"),   TokenType::TRUE);
        keywords.insert(String::from("var"),    TokenType::VAR);
        keywords.insert(String::from("while"),  TokenType::WHILE);

        keywords
    }
}


impl Scanner {
    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.add_token(TokenType::EOF);

        &self.tokens
    }


    fn scan_token(&mut self){
        let cur_token = self.advance();

        match cur_token {
            '(' => self.add_token(TokenType::LEFTPAREN),
            ')' => self.add_token(TokenType::RIGHTPAREN),
            '{' => self.add_token(TokenType::LEFTBRACE),
            '}' => self.add_token(TokenType::LEFTBRACE),
            ',' => self.add_token(TokenType::COMMA),
            '.' => self.add_token(TokenType::DOT),
            '+' => self.add_token(TokenType::PLUS),
            '-' => self.add_token(TokenType::MINUS),
            '*' => self.add_token(TokenType::STAR),
            '/' => self.add_token(TokenType::SLASH),
            ';' => self.add_token(TokenType::SEMICOLON),
            '!' => {
                        let token = if self.match_token('='){TokenType::BANGEQUAL} else {TokenType::EQUAL};
                        self.add_token(token);
                    },
            '=' => {
                        let token = if self.match_token('='){TokenType::EQUALEQUAL} else {TokenType::EQUAL};
                        self.add_token(token);
                    },
            '>' => {
                        let token = if self.match_token('='){TokenType::GREATEREQUAL} else {TokenType::GREATER};
                        self.add_token(token);
                    },
            '<' => {
                        let token = if self.match_token('='){TokenType::LESSEQUAL} else {TokenType::LESS};
                        self.add_token(token);
                    },
            ' ' | '\r' |'\t' => {},
            '\n' => self.line += 1,
            '"' => self.string(),
            _ => {
                if self.is_digit(cur_token) {
                    self.number();
                }
                else if self.is_alpha(cur_token) {
                    self.identifier();
                }
                else {
                    panic!("Invalid token")
                }
            }
        }
    }


    fn match_token(&mut self, expected_token: char) -> bool {
        if self.is_end() || self.peek() != expected_token { //either we reach end or we did not
            //found what we wanted
            return false;
        }
        else{
            self.current += 1;
            return true;
        }
    }

    fn advance(&mut self) -> char {
        let token = self.source.chars().nth(self.current).unwrap();
        (*self).current += 1;
        token
    }


    fn string(&mut self) {
        while self.peek() != '"' && !self.is_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_end(){ panic!("Undetermined string") }

        self.advance();
        
        let value: String = self.substr();
        self.add_token_verbose(TokenType::STRING, Some(Box::new(value)));
    }

    fn number(&mut self) {
        while self.is_digit(self.peek()) { self.advance(); }
     
        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            self.advance();

            while self.is_digit(self.peek()) { self.advance(); }
        }
        let val = self.substr().parse::<usize>();
        self.add_token_verbose(TokenType::NUMBER, Some(Box::new(val)));
    }

    fn identifier(&mut self) {
        while self.is_alphanumeric(self.peek()) {
            self.advance();
        }

        let text: String = self.substr();

        let token_type: TokenType = if let Some(_type) = self.keywords.get(&text) {
            _type.clone()
        }
        else { TokenType::IDENTIFIER };

        self.add_token(token_type);
    }
}

impl Scanner {
    fn is_end(&self) -> bool {
        if self.current >= self.source.len() {
            true
        }
        else {
            false
        }
    }

    fn add_token(&mut self, token: TokenType) -> () {
        self.add_token_verbose(token, None);
    }

    fn add_token_verbose(&mut self, token_type: TokenType, literal: Option<Box<dyn Any>>){
        let token = Token {
            line: self.line,
            token_type,
            lexeme: self.substr(),
            literal: if let Some(lit) = literal { lit } else { Box::new(TokenType::NIL) }
        };

        self.tokens.push(token);
    }

    fn substr(&self) -> String {
        self.source.substr(self.start, self.current)
    }

    fn peek(&self) -> char {
        if self.current >= self.source.len() {return '\0';}
        self.source.chars().nth(self.current).unwrap()
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {return '\0';}

        self.source.chars().nth(self.current + 1).unwrap()
    }

    fn is_alphanumeric(&self, token: char) -> bool {
        return self.is_digit(token) || self.is_alpha(token);
    }

    fn is_digit(&self, token: char) -> bool {
        return token >= '0' && token <= '9';
    }

    fn is_alpha(&self, token: char) -> bool {
        return (token >= 'a' && token <= 'z') ||
            (token >= 'A' && token <= 'Z') ||
            token == '_';
    }
}
