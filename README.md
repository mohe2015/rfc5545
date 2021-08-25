# rfc5545

https://datatracker.ietf.org/doc/html/rfc5545

## Building

https://doc.rust-lang.org/rustc/linker-plugin-lto.html

RUSTFLAGS="-C target-cpu=native -Clinker-plugin-lto" cargo build --release

RUSTFLAGS="-C target-cpu=native -Clinker-plugin-lto -Clinker=clang -Clink-arg=-fuse-ld=lld" cargo build --release

https://rust.godbolt.org/#g:!((g:!((g:!((h:codeEditor,i:(fontScale:14,fontUsePx:'0',j:1,lang:rust,selection:(endColumn:2,endLineNumber:8,positionColumn:2,positionLineNumber:8,selectionStartColumn:2,selectionStartLineNumber:8,startColumn:2,startLineNumber:8),source:'use+std::%7Berror::Error,+io::%7BWrite,+stdout%7D%7D%3B%0A%0Apub+fn+main()+-%3E+Result%3C(),+Box%3Cdyn+Error%3E%3E+%7B%0A++++let+mut+stdout+%3D+stdout()%3B%0A++++stdout.write_all(b%22HELLOOO%22)%3F%3B%0A++++stdout.write_all(b%22WORLD%22)%3F%3B%0A++++Ok(())%0A%7D'),l:'5',n:'0',o:'Rust+source+%231',t:'0')),k:50,l:'4',n:'0',o:'',s:0,t:'0'),(g:!((h:compiler,i:(compiler:r1540,filters:(b:'0',binary:'1',commentOnly:'0',demangle:'0',directives:'0',execute:'1',intel:'0',libraryCode:'0',trim:'1'),flagsViewOpen:'1',fontScale:14,fontUsePx:'0',j:1,lang:rust,libs:!(),options:'-C++++++++++++++++opt-level%3D3+',selection:(endColumn:1,endLineNumber:1,positionColumn:1,positionLineNumber:1,selectionStartColumn:1,selectionStartLineNumber:1,startColumn:1,startLineNumber:1),source:1),l:'5',n:'0',o:'rustc+1.54.0+(Rust,+Editor+%231,+Compiler+%231)',t:'0')),k:50,l:'4',n:'0',o:'',s:0,t:'0')),l:'2',n:'0',o:'',t:'0')),version:4

TODO https://doc.rust-lang.org/rustc/profile-guided-optimization.html

## Goals

The main goal is that we create as efficient code as possible.

Currently it doesn't seem like e.g. write_all calls are merged.

Parsing should have an event-based parser and a simpler getter based interface.