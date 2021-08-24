use lazy_static::lazy_static;
use regex::Regex;
use std::{error::Error, io::Write};

// https://datatracker.ietf.org/doc/html/rfc5545#section-3.1
// ABNF: https://datatracker.ietf.org/doc/html/rfc5234

pub struct ContentLinesParser {}

enum ContentLinesBuilderState {
    Start,
    MoreParamValues,
}

pub struct ContentLinesBuilder<W: Write> {
    output: W,
    state: ContentLinesBuilderState,
}

// x-name seems to be a subset of iana-token
lazy_static! {
    static ref IANA_TOKEN_REGEX: Regex = Regex::new("[A-Za-z0-9-]+").unwrap();
}

impl<W: Write> ContentLinesBuilder<W> {
    pub fn new(output: W) -> Self {
        Self {
            output,
            state: ContentLinesBuilderState::Start,
        }
    }

    // TODO FIXME maybe check in debugging builds that these methods are called in the right order. Or try to ensure this at the type-level
    // this could probably also be done for the iana_tokens by creating a custom class that checks the condition on creation and just wraps a &str
    pub fn write_name(&mut self, name: &str) -> Result<(), Box<dyn Error>> {
        // TODO FIXME depending on if this is user input always check this
        assert!(IANA_TOKEN_REGEX.is_match(name));
        self.output.write_all(name.as_bytes())?;
        Ok(())
    }

    pub fn write_param_name(&mut self, param_name: &str) -> Result<(), Box<dyn Error>> {
        assert!(IANA_TOKEN_REGEX.is_match(param_name));
        self.output.write_all(b";")?;
        self.output.write_all(param_name.as_bytes())?;
        self.state = ContentLinesBuilderState::Start;
        Ok(())
    }

    pub fn write_param_value(&mut self, param_value: &str) -> Result<(), Box<dyn Error>> {
        // TODO FIXME validate param_value
        match self.state {
            ContentLinesBuilderState::Start => {
                self.output.write_all(b"=")?;
                self.state = ContentLinesBuilderState::MoreParamValues;
            }
            ContentLinesBuilderState::MoreParamValues => {
                self.output.write_all(b",")?;
            }
        };
        self.output.write_all(param_value.as_bytes())?;
        Ok(())
    }

    pub fn write_value(&mut self, value: &str) -> Result<(), Box<dyn Error>> {
        self.output.write_all(b":")?;
        // FIXME validate value
        self.output.write_all(value.as_bytes())?;
        self.output.write_all(b"\r\n")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::{error::Error, io::stdout, io::Write};

    use super::ContentLinesBuilder;

    #[test]
    fn it_works() -> Result<(), Box<dyn Error>> {
        let mut content_lines_builder = ContentLinesBuilder::new(Vec::<u8>::new());
        content_lines_builder.write_name("TEST")?;
        content_lines_builder.write_param_name("TEST")?;
        content_lines_builder.write_param_value("TEST")?;
        content_lines_builder.write_value("TEST")?;
        stdout().write(&content_lines_builder.output)?;
        Ok(())
    }
}
