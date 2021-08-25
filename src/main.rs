use std::{error::Error, io::{Write, stdout}};

use rfc5545::content_lines::ContentLinesBuilder;


fn main() {
    print!("{}{}{}", "hello", "how", "are");

    /*let stdout = stdout();
    let mut locked_stdout = stdout.lock();

    // this also doesnt seem to work as this is technically probably not equal
    // this could probably work if we instead work with strings directly or so because they wouldn't have strange side-effects
    locked_stdout.write_all(b"HELLOOO")?;
    locked_stdout.write_all(b"WORLD")?;
*/

    /*let mut content_lines_builder = ContentLinesBuilder::new(stdout());
    content_lines_builder.write_name("TEST")?;
    content_lines_builder.write_param_name("TEST")?;
    content_lines_builder.write_param_value("TEST")?;
    content_lines_builder.write_value("TEST")?;
    Ok(())*/
}