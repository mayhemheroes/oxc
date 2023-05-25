#![no_main]

use libfuzzer_sys::fuzz_target;

use oxc_allocator::Allocator;
use oxc_parser::Parser;
use oxc_semantic::SemanticBuilder;
use oxc_span::SourceType;

fuzz_target!(|data: (bool, bool, bool, bool, &str)| {
    let (is_ts, is_script, is_jsx, always_strict, code) = data;
    let source_type = SourceType::default().with_typescript(is_ts).with_script(is_script).with_jsx(is_jsx).with_always_strict(always_strict);

    let allocator = Allocator::default();
    let ret = Parser::new(&allocator, code, source_type).parse();

    if ret.errors.is_empty() {
        let program = allocator.alloc(ret.program);
        let _semantic_ret = SemanticBuilder::new(code, source_type, &ret.trivias).build(program);
    }
});
