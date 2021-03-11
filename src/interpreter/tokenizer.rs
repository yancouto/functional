pub type Variable = char;
pub type Constant = String;

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    // x, y
    Variable(Variable),
    // hello, goodbye
    Constant(Constant),
    // :
    Colon,
    // (
    OpenPar,
    // )
    ClosePar,
}

#[derive(Debug)]
pub enum TokenizeError {
    UnknownCharacter(char),
}

fn tokenize_vec<S: IntoIterator<Item = char>>(str: S) -> Result<Vec<Token>, TokenizeError> {
    let mut ans = vec![];
    let mut cur_word: Vec<char> = vec![];
    let flush = |cur_word: &mut Vec<char>, ans: &mut Vec<Token>| {
        if cur_word.len() == 1 {
            ans.push(Token::Variable(cur_word[0]));
        } else if cur_word.len() > 1 {
            ans.push(Token::Constant(cur_word.iter().collect()));
        }
        cur_word.clear();
    };
    for c in str.into_iter() {
        if c.is_ascii_alphabetic() {
            cur_word.push(c);
        } else {
            flush(&mut cur_word, &mut ans);
            if !c.is_whitespace() {
                match c {
                    ':' => ans.push(Token::Colon),
                    '(' => ans.push(Token::OpenPar),
                    ')' => ans.push(Token::ClosePar),
                    _ => return Err(TokenizeError::UnknownCharacter(c)),
                }
            }
        }
    }
    flush(&mut cur_word, &mut ans);
    Ok(ans)
}

pub fn tokenize<S: IntoIterator<Item = char>>(
    str: S,
) -> Result<impl Iterator<Item = Token>, TokenizeError> {
    // TODO: We can make a proper iterator that goes through the string as needed
    Ok(tokenize_vec(str)?.into_iter())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            tokenize("a: asd()".chars())
                .unwrap()
                .collect::<Vec<Token>>(),
            vec![
                Token::Variable('a'),
                Token::Colon,
                Token::Constant("asd".to_string()),
                Token::OpenPar,
                Token::ClosePar
            ]
        );
    }
}
