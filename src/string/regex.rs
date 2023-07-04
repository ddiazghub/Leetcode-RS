use std::iter::Peekable;

#[derive(Clone, Copy)]
enum Token {
    Literal(char),
    Repeat(char)
}

struct Tokens<I: Iterator<Item = char>>(Peekable<I>);

impl <I: Iterator<Item = char>> Iterator for Tokens<I> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        match (self.0.next(), self.0.peek()) {
            (Some(token), Some(&'*')) => {
                self.0.next();
                Some(Token::Repeat(token))
            },
            (Some(token), _) => Some(Token::Literal(token)),
            (None, _) => None
        }
    }
}

fn skip_until<T, F: Fn(&T) -> bool>(iter: &mut Peekable<impl Iterator<Item = T>>, predicate: F) {
    while let Some(element) = iter.peek() {
        if predicate(element) {
            break;
        }

        iter.next();
    }
}

pub fn is_match(s: String, p: String) -> bool {
    let mut chars = s.chars().peekable();
    let mut tokens = Tokens(p.chars().peekable()).peekable();

    while let Some(token) = tokens.next() {
        let matches = match token {
            Token::Literal('.') => chars.next().is_some(),
            Token::Literal(token) => chars.next().map_or(false, |literal| token == literal),
            Token::Repeat('.') => chars.all(|_| true),
            Token::Repeat(token) => {
                skip_until(&mut chars, |&literal| token != literal);
                skip_until(&mut tokens, |&t| {
                    if let Token::Literal(t) = t {
                        token != t
                    } else {
                        true
                    }
                });

                true
            }
        };

        if !matches {
            return false;
        }
    }

    chars.next().is_none()
}

#[cfg(test)]
mod tests {
    use super::is_match;

    #[test]
    fn test1() {
        let result = is_match("aa".to_string(), "a".to_string());
        assert_eq!(result, false);
    }

    #[test]
    fn test2() {
        let result = is_match("aa".to_string(), "a*".to_string());
        assert_eq!(result, true);
    }

    #[test]
    fn test3() {
        let result = is_match("ab".to_string(), ".*".to_string());
        assert_eq!(result, true);
    }

    #[test]
    fn test4() {
        let result = is_match("aaa".to_string(), "a*a".to_string());
        assert_eq!(result, true);
    }

    #[test]
    fn test5() {
        let result = is_match("aaa".to_string(), "ab*a*c*a".to_string());
        assert_eq!(result, true);
    }
}
