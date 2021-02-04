pub mod language {

    #[derive(Debug)]
    pub struct Token {
        pub token_val: String,
    }

    pub fn lexer(text: &str) -> Vec<Token> {
        let mut vec: Vec<Token> = Vec::new();
        let mut token: Vec<char> = Vec::new();

        for ch in text.chars() {
            match ch {
                ' ' | '\n' => {
                    let tok: String = token.iter().cloned().collect();
                    if !tok.is_empty() && tok.len() > 0 {
                        vec.push(Token { token_val: tok });
                    }
                    token.clear();
                    continue;
                }
                _ => {
                    token.push(ch);
                }
            }
        }
        let tok: String = token.iter().cloned().collect();
        if !tok.is_empty() && tok.len() > 0 {
            vec.push(Token { token_val: tok });
        }
        vec
    }

    // checks to see if s is a number
    fn is_number(s: &String) -> bool {
        for i in s.chars() {
            if !i.is_digit(10) {
                //radix = decimal
                return false;
            }
        }
        true
    }

    //parse string s to number
    fn to_number(s: &String) -> i32 {
        let s: i32 = s.parse().expect("Unable to parse to number");
        s
    }

    //map the operator/mnemonic to vm code
    fn map_to_number(s: &String) -> i32 {
        match s.as_str() {
            "+" => 0x40000001,
            "-" => 0x40000002,
            "*" => 0x40000003,
            "/" => 0x40000004,
            _ => {
                panic!("Inavlid instruction");
            }
        }
    }

    //parse the tokens into vm instructions
    pub fn parser(token: Vec<Token>) -> Vec<i32> {
        let mut vec: Vec<i32> = Vec::new();
        for i in token {
            if is_number(&i.token_val) {
                vec.push(to_number(&i.token_val));
            } else {
                vec.push(map_to_number(&i.token_val));
            }
        }
        vec.push(0x40000000);
        vec
    }
}
