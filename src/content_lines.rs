extern crate alloc;

use core::fmt::Write;
use std::error::Error;

use alloc::boxed::Box;
use lazy_static::lazy_static;
use regex::Regex;

// https://datatracker.ietf.org/doc/html/rfc5545#section-3.1
// ABNF: https://datatracker.ietf.org/doc/html/rfc5234

pub struct ContentLinesParser {}

enum ContentLinesBuilderState {
    Start,
    MoreParamValues,
}

pub struct ContentLinesBuilder<W: Write> {
    pub output: W,
    state: ContentLinesBuilderState,
}

// x-name seems to be a subset of iana-token
/*lazy_static! {
    static ref IANA_TOKEN_REGEX: Regex = Regex::new("[A-Za-z0-9-]+").unwrap();
}*/

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
        //assert!(IANA_TOKEN_REGEX.is_match(name));
        self.output.write_str(name)?;
        Ok(())
    }

    pub fn write_param_name(&mut self, param_name: &str) -> Result<(), Box<dyn Error>> {
        //assert!(IANA_TOKEN_REGEX.is_match(param_name));
        self.output.write_str(";")?;
        self.output.write_str(param_name)?;
        self.state = ContentLinesBuilderState::Start;
        Ok(())
    }

    pub fn write_param_value(&mut self, param_value: &str) -> Result<(), Box<dyn Error>> {
        // TODO FIXME validate param_value
        match self.state {
            ContentLinesBuilderState::Start => {
                self.output.write_str("=")?;
                self.state = ContentLinesBuilderState::MoreParamValues;
            }
            ContentLinesBuilderState::MoreParamValues => {
                self.output.write_str(",")?;
            }
        };
        self.output.write_str(param_value)?;
        Ok(())
    }

    pub fn write_value(&mut self, value: &str) -> Result<(), Box<dyn Error>> {
        self.output.write_str(":")?;
        // FIXME validate value
        self.output.write_str(value)?;
        self.output.write_str("\r\n")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use std::error::Error;

    use alloc::{boxed::Box, vec::Vec};

    use super::ContentLinesBuilder;

    #[test]
    fn it_works() -> Result<(), Box<dyn Error>> {
        let mut content_lines_builder = ContentLinesBuilder::new(String::new());
        content_lines_builder.write_name("TEST")?;
        content_lines_builder.write_param_name("TEST")?;
        content_lines_builder.write_param_value("TEST")?;
        content_lines_builder.write_value("TEST")?;
        assert_eq!("TEST;TEST=TEST:TEST\r\n", content_lines_builder.output);
        Ok(())
    }
}
