use crate::isa::{Instructions, Items};

pub struct VectAsmCompiler {
    input: String,
    result: Vec<Items>,
}

impl VectAsmCompiler {
    pub fn init() -> Self {
        Self {
            input: String::new(),
            result: Vec::new(),
        }
    }

    pub fn load(&mut self, string: String) {
        self.input = string;
    }

    pub fn compile(&mut self) -> Result<(), String> {
        let tokens = lex(&self.input)?;
        let mut cursor = TokenCursor::new(tokens);
        self.result = parse_script(&mut cursor)?;
        if cursor.has_remaining() {
            return Err(format!(
                "unexpected token {:?} after script",
                cursor.peek()
            ));
        }
        Ok(())
    }

    pub fn result(&self) -> &Vec<Items> {
        &self.result
    }

    pub fn take_result(self) -> Vec<Items> {
        self.result
    }
}

pub fn compile_vect_asm(source: &str) -> Result<Vec<Items>, String> {
    let mut compiler = VectAsmCompiler::init();
    compiler.load(source.to_string());
    compiler.compile()?;
    Ok(compiler.take_result())
}

#[derive(Debug, Clone, PartialEq)]
enum Token {
    LParen,
    RParen,
    Comma,
    Ident(String),
    Number(i32),
}

struct TokenCursor {
    tokens: Vec<Token>,
    index: usize,
}

impl TokenCursor {
    fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, index: 0 }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.index)
    }

    fn bump(&mut self) -> Option<Token> {
        let token = self.tokens.get(self.index).cloned();
        if token.is_some() {
            self.index += 1;
        }
        token
    }

    fn expect(&mut self, expected: Token) -> Result<(), String> {
        match self.bump() {
            Some(token) if token == expected => Ok(()),
            Some(token) => Err(format!("expected {:?}, found {:?}", expected, token)),
            None => Err(format!("expected {:?}, found end of input", expected)),
        }
    }

    fn has_remaining(&self) -> bool {
        self.index < self.tokens.len()
    }
}

fn lex(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            '(' => {
                chars.next();
                tokens.push(Token::LParen);
            }
            ')' => {
                chars.next();
                tokens.push(Token::RParen);
            }
            ',' => {
                chars.next();
                tokens.push(Token::Comma);
            }
            ' ' | '\t' | '\r' | '\n' | ';' => {
                chars.next();
            }
            '-' | '0'..='9' => {
                tokens.push(read_number(&mut chars)?);
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                tokens.push(read_ident(&mut chars)?);
            }
            _ => {
                return Err(format!("unexpected character '{ch}'"));
            }
        }
    }

    Ok(tokens)
}

fn read_number(chars: &mut std::iter::Peekable<std::str::Chars<'_>>) -> Result<Token, String> {
    let mut text = String::new();

    if matches!(chars.peek(), Some('-')) {
        text.push(chars.next().unwrap());
    }

    let mut has_digit = false;
    while let Some(&ch) = chars.peek() {
        if ch.is_ascii_digit() {
            has_digit = true;
            text.push(chars.next().unwrap());
        } else {
            break;
        }
    }

    if !has_digit {
        return Err("invalid number literal".to_string());
    }

    let value = text
        .parse::<i32>()
        .map_err(|_| format!("number out of range: {text}"))?;
    Ok(Token::Number(value))
}

fn read_ident(chars: &mut std::iter::Peekable<std::str::Chars<'_>>) -> Result<Token, String> {
    let mut text = String::new();

    while let Some(&ch) = chars.peek() {
        if ch.is_ascii_alphanumeric() || ch == '_' {
            text.push(chars.next().unwrap());
        } else {
            break;
        }
    }

    Ok(Token::Ident(text))
}

fn parse_script(cursor: &mut TokenCursor) -> Result<Vec<Items>, String> {
    match cursor.peek() {
        Some(Token::LParen) => parse_group(cursor),
        Some(token) => Err(format!("script must start with '(', found {:?}", token)),
        None => Err("empty input".to_string()),
    }
}

fn parse_group(cursor: &mut TokenCursor) -> Result<Vec<Items>, String> {
    cursor.expect(Token::LParen)?;

    let mut items = Vec::new();

    if matches!(cursor.peek(), Some(Token::RParen)) {
        cursor.expect(Token::RParen)?;
        return Ok(items);
    }

    loop {
        items.push(parse_item(cursor)?);

        match cursor.peek() {
            Some(Token::Comma) => {
                cursor.bump();
            }
            Some(Token::RParen) => {
                cursor.expect(Token::RParen)?;
                break;
            }
            Some(token) => {
                return Err(format!("expected ',' or ')', found {:?}", token));
            }
            None => {
                return Err("unexpected end of input inside group".to_string());
            }
        }
    }

    Ok(items)
}

fn parse_item(cursor: &mut TokenCursor) -> Result<Items, String> {
    match cursor.peek() {
        Some(Token::LParen) => {
            let inner = parse_group(cursor)?;
            Ok(Items::Recursion(Box::new(inner)))
        }
        Some(Token::Number(value)) => {
            let value = *value;
            cursor.bump();
            Ok(Items::Number(value))
        }
        Some(Token::Ident(name)) => {
            let name = name.clone();
            cursor.bump();
            Ok(Items::Element(parse_instruction(&name)?))
        }
        Some(token) => Err(format!("unexpected token {:?}", token)),
        None => Err("unexpected end of input".to_string()),
    }
}

fn parse_instruction(name: &str) -> Result<Instructions, String> {
    match name {
        "add" => Ok(Instructions::Add),
        "jump" => Ok(Instructions::Jump),
        "lgr" => Ok(Instructions::Lgr),
        _ => Err(format!("unknown instruction '{name}'")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::Vectvm;

    #[test]
    fn compiles_test_vect_layout() {
        let script = compile_vect_asm("(add, (add, 1, (add, 2, 3)), 4)").unwrap();

        assert!(matches!(script.first(), Some(Items::Element(Instructions::Add))));
        assert!(matches!(script.last(), Some(Items::Number(4))));
        assert_eq!(script.len(), 3);

        let middle = match &script[1] {
            Items::Recursion(inner) => inner.as_slice(),
            _ => panic!("expected recursion as second item"),
        };
        assert!(matches!(middle.first(), Some(Items::Element(Instructions::Add))));
        assert!(matches!(middle.get(1), Some(Items::Number(1))));
        assert!(matches!(middle.get(2), Some(Items::Recursion(_))));
    }

    #[test]
    fn script_order_matches_rewind() {
        let script = compile_vect_asm("(add, 1, 2)").unwrap();
        let mut vm = Vectvm::init();
        vm.push(script);
        assert!(matches!(vm.evaluate(), Items::Number(3)));
    }

    #[test]
    fn evaluates_test_vect() {
        let script = compile_vect_asm("(add, (add, 1, (add, 2, 3)), 4)").unwrap();
        let mut vm = Vectvm::init();
        vm.push(script);
        let result = vm.evaluate();
        assert!(matches!(result, Items::Number(10)));
    }
}
