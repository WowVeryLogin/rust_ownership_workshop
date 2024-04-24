struct Logger {}

struct ParserMessage {
    autocomplete: String,
}

struct Input<'a> {
    input: String,
    logger: &'a Logger,
}

impl Input<'_> {
    fn read(&mut self) -> String {
        let whitespace = self.input.find(' ').unwrap_or(self.input.len());
        let expression: String = self.input.drain(..whitespace).collect();
        if !self.input.is_empty() {
            self.input.drain(..1);
        }

        // Your code goes here, if Parser asked us to autocomplete, then autocomplete;
        // if parser gave autocomplete_string {
        //      if !expression.ends_with(autocomplete_string) {
        //          return t + autocomplete_string;
        //      }
        // }
        expression
    }
}

struct Lexer<'a> {
    // do not modify Lexer
    input: Input<'a>,
}

impl Lexer<'_> {
    fn call(&mut self) -> String {
        let from_input = self.input.read();
        if from_input.starts_with('{') {
            return "block_start:".to_owned() + &from_input;
        }
        if from_input.is_empty() {
            return "end".to_owned();
        }
        from_input
    }
}

struct Parser<'a> {
    lexer: Lexer<'a>,
    logger: &'a Logger,
}

impl Parser<'_> {
    fn parse(&mut self) -> String {
        let mut parsed = vec![];
        let mut value = self.lexer.call();

        while &value != "end" {
            let mut v = value;
            if v.starts_with("block_start:") {
                let fixed_v = v.strip_prefix("block_start:").unwrap();

                // your code goes here: somehow ask input to autocomplete the next block with "}"

                v = fixed_v.to_owned();
            }
            parsed.push(v);
            value = self.lexer.call();
        }

        parsed.join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "{ab aba ba {bb bb} {ab aa".to_owned();
        let expected = "{ab aba} ba {bb bb} {ab aa}".to_owned();

        let logger = &Logger {};
        let mut p = Parser {
            logger,
            lexer: Lexer {
                input: Input { input, logger },
            },
        };
        assert_eq!(p.parse(), expected);
    }
}
