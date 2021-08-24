# rfc5545

https://datatracker.ietf.org/doc/html/rfc5545

# Goals

The main goal is that we create as efficient code as possible.

Currently it doesn't seem like e.g. write_all calls are merged.

Parsing should have an event-based parser and a simpler getter based interface.