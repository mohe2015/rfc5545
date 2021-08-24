use std::{error::Error, io::{Write, stdout}};

use rfc5545::content_lines::ContentLinesBuilder;


fn main() -> Result<(), Box<dyn Error>> {
    let mut stdout = stdout();
    // this also doesnt seem to work as this is technically probably not equal
    stdout.write_all(b"HELLOOO")?;
    stdout.write_all(b"WORLD")?;
    Ok(())
    /*let mut content_lines_builder = ContentLinesBuilder::new(stdout());
    content_lines_builder.write_name("TEST")?;
    content_lines_builder.write_param_name("TEST")?;
    content_lines_builder.write_param_value("TEST")?;
    content_lines_builder.write_value("TEST")?;
    Ok(())*/
}