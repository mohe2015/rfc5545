// https://datatracker.ietf.org/doc/html/rfc5545#section-4

// examples show that there can be nested data (so we need some parsing part for that)

// as parsing would optimally return an object the easiest way would be to also build the ical from that

// a more efficient parsing would be to only extract what we need

// potentially a pull based event based parser? - but then we would need to properly write all data back so this doesn't work.

// so really just parsing into an efficient data format is probably the most useful thing

// maybe also just parse into a common datastructure and then provide some custom getters and setters

// I think the underlying datastructures should be really basic and then build nice methods over it

// there is a component system that we should properly implement

// https://github.com/migmedia/ical-rs/blob/master/src/parser/ical/component.rs
// seems like a good start