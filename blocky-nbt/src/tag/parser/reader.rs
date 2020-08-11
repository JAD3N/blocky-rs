pub struct Reader {
    text: String,
    chars: Vec<char>,
    position: usize,
}

impl Reader {
    pub fn new<S: Into<String>>(s: S) -> Self {
        let text = s.into();
        let chars = text.chars().collect();

        Self {
            text,
            chars,
            position: 0,
        }
    }

    pub fn position(&self) -> usize {
        self.position
    }

    pub fn set_position(&mut self, position: usize) {
        self.position = position;
    }

    pub fn remaining(&self) -> usize {
        self.len() - self.position
    }

    pub fn len(&self) -> usize {
        self.text.len()
    }

    pub fn has_remaining(&self, n: usize) -> bool {
        self.remaining() >= n
    }

    pub fn done(&self) -> bool {
        !self.has_remaining(1)
    }

    pub fn peek_nth(&self, n: usize) -> anyhow::Result<char> {
        self.chars.get(self.position + n)
            .map(|chr| *chr)
            .ok_or(anyhow::anyhow!("not enough chars"))
    }

    pub fn peek(&self) -> anyhow::Result<char> {
        self.peek_nth(0)
    }

    pub fn read_nth(&mut self, n: usize) -> anyhow::Result<char> {
        let chr = self.peek_nth(n)?;
        self.position += 1 + n;
        Ok(chr)
    }

    pub fn read(&mut self) -> anyhow::Result<char> {
        self.read_nth(0)
    }

    pub fn skip(&mut self) {
        self.position += 1;
    }

    pub fn skip_whitespace(&mut self) -> anyhow::Result<()> {
        while !self.done() {
            if self.peek()?.is_whitespace() {
                self.skip();
            } else {
                break;
            }
        }

        Ok(())
    }

    pub fn expect(&mut self, chr: char) -> anyhow::Result<()> {
        let read_chr = self.read()?;

        if read_chr != chr {
            anyhow::bail!("expected {} but got {}", chr, read_chr);
        }

        Ok(())
    }

    pub fn is_quote(chr: char) -> bool {
        chr == '"' || chr == '\''
    }

    pub fn is_allowed_in_unquoted_string(chr: char) -> bool {
        chr >= '0' && chr <= '9'
            || chr >= 'A' && chr <= 'Z'
            || chr >= 'a' && chr <= 'z'
            || chr == '_'
            || chr == '-'
            || chr == '.'
            || chr == '+'
    }

    pub fn read_unquoted_string(&mut self) -> anyhow::Result<String> {
        let start = self.position;

        while !self.done() && Self::is_allowed_in_unquoted_string(self.peek()?) {
            self.skip();
        }

        Ok(String::from(&self.text[start..self.position]))
    }

    pub fn read_quoted_string(&mut self) -> anyhow::Result<String> {
        if self.done() {
            return Ok(String::new());
        }

        let chr = self.peek()?;

        if !Self::is_quote(chr) {
            anyhow::bail!("invalid quote char");
        }

        self.skip();
        self.read_string_until(chr)
    }

    pub fn read_string_until(&mut self, chr: char) -> anyhow::Result<String> {
        let mut builder = String::new();
        let mut escaped = false;

        while !self.done() {
            let read_chr = self.read()?;

            if escaped {
                if read_chr == chr || read_chr == '\\' {
                    builder.push(read_chr);
                    escaped = false;
                } else {
                    self.position -= 1;
                    anyhow::bail!("invalid escape in string");
                }
            } else if read_chr == '\\' {
                escaped = true;
            } else if read_chr == chr {
                return Ok(builder);
            } else {
                builder.push(read_chr);
            }
        }

        Err(anyhow::anyhow!("unexpected end of string"))
    }

    pub fn read_string(&mut self) -> anyhow::Result<String> {
        if self.done() {
            Ok(String::new())
        } else {
            let chr = self.peek()?;

            if Self::is_quote(chr) {
                self.read_quoted_string()
            } else {
                self.read_unquoted_string()
            }
        }
    }
}
