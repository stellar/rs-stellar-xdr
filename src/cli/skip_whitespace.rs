use std::io::Read;

/// Forwards read operations to the wrapped object, skipping over any
/// whitespace.
pub struct SkipWhitespace<R: Read> {
    pub inner: R,
}

impl<R: Read> SkipWhitespace<R> {
    pub fn new(inner: R) -> Self {
        SkipWhitespace { inner }
    }
}

impl<R: Read> Read for SkipWhitespace<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let n = self.inner.read(buf)?;

        let mut written = 0;
        for read in 0..n {
            if !buf[read].is_ascii_whitespace() {
                buf[written] = buf[read];
                written += 1;
            }
        }

        Ok(written)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        struct Test {
            input: &'static [u8],
            output: &'static [u8],
        }
        let tests = [
            Test {
                input: b"",
                output: b"",
            },
            Test {
                input: b" \n\t\r",
                output: b"",
            },
            Test {
                input: b"a c",
                output: b"ac",
            },
            Test {
                input: b"ab cd",
                output: b"abcd",
            },
            Test {
                input: b" ab \n cd ",
                output: b"abcd",
            },
        ];
        for (i, t) in tests.iter().enumerate() {
            let mut skip = SkipWhitespace::new(t.input);
            let mut output = Vec::new();
            skip.read_to_end(&mut output).unwrap();
            assert_eq!(output, t.output, "#{i}");
        }
    }
}
