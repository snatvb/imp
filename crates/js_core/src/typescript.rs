use oxc_allocator::Allocator;
use oxc_codegen::Codegen;
use oxc_parser::{ParseOptions, Parser};
use oxc_semantic::SemanticBuilder;
use oxc_span::SourceType;
use oxc_transformer::{TransformOptions, Transformer, TypeScriptOptions};
use std::{cell::RefCell, path::Path, sync::OnceLock};

static DUMMY_PATH: OnceLock<&'static Path> = OnceLock::new();
thread_local! {
    static ALOCATOR_POOL: RefCell<Allocator> = RefCell::new(Allocator::default());
}

#[tracing::instrument(level = "debug", skip_all, fields(input_len = ts_code.len()))]
pub fn strip_types_fast(ts_code: &str, ts_options: TypeScriptOptions) -> Result<String, String> {
    let result = ALOCATOR_POOL.with(|allocator_cell| {
        let mut allocator = allocator_cell.borrow_mut();
        allocator.reset();

        let source_type = SourceType::ts();

        let parser = Parser::new(&allocator, ts_code, source_type).with_options(ParseOptions {
            allow_return_outside_function: true,
            ..ParseOptions::default()
        });
        let parsed = parser.parse();

        if !parsed.errors.is_empty() {
            tracing::debug!(errors = parsed.errors.len(), "TS parse errors");
            return Err(format!("TS Syntax Errors: {:?}", parsed.errors));
        }
        tracing::trace!("TS parsed");

        let mut program = parsed.program;

        let semantic_return = SemanticBuilder::new().with_cfg(false).build(&program);
        let scoping = semantic_return.semantic.into_scoping();
        tracing::trace!("TS semantic built");

        let options = TransformOptions {
            typescript: ts_options,
            ..Default::default()
        };
        let dummy_path = DUMMY_PATH.get_or_init(|| Path::new("mod.ts"));
        Transformer::new(&allocator, dummy_path, &options)
            .build_with_scoping(scoping, &mut program);
        tracing::trace!("TS transformed");

        let codegen_return = Codegen::new().build(&program);
        tracing::debug!(output_len = codegen_return.code.len(), "TS codegen done");

        Ok(codegen_return.code)
    });

    #[cfg(debug_assertions)]
    if let Ok(ref out) = result {
        tracing::info!(output_len = out.len(), "TS stripped");
    }

    result
}

pub fn strip_types_fast_default(ts_code: &str) -> Result<String, String> {
    strip_types_fast(ts_code, Default::default())
}

const EXTS: &[&str] = &["ts", "mts", "cts", "tsx"];

#[tracing::instrument(level = "trace", skip(filepath), fields(path = %filepath.as_ref().display()))]
#[inline(always)]
pub fn is_ts_ext<P: AsRef<Path>>(filepath: P) -> bool {
    let result = filepath
        .as_ref()
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| EXTS.contains(&ext))
        .unwrap_or(false);
    tracing::trace!(is_ts = result, "ext check");
    result
}
