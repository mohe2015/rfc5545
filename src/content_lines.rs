// https://datatracker.ietf.org/doc/html/rfc5545#section-3.1

struct ContentLinesParser {

}



struct ContentLinesBuilder {
    output: Write
}

impl ContentLinesBuilder {

    fn write_name(&self, name: str) {
        self.output
    }
}