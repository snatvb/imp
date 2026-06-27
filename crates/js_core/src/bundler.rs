use std::collections::{HashMap, HashSet, VecDeque};
use std::path::Path;

use oxc_allocator::Allocator;
use oxc_parser::{ParseOptions, Parser};
use oxc_span::SourceType;
use serde_json;

use embed::Bundle;

use crate::resolver::Resolver;
use crate::typescript;

fn extract_imports(source: &str, file_path: &str) -> Vec<String> {
    let allocator = Allocator::default();
    let source_type = SourceType::mjs();
    let parser = Parser::new(&allocator, source, source_type).with_options(ParseOptions {
        allow_return_outside_function: true,
        ..ParseOptions::default()
    });
    let parsed = parser.parse();
    if !parsed.errors.is_empty() {
        tracing::warn!(
            file = file_path,
            errors = ?parsed.errors,
            "bundler: parse errors, imports may be missed"
        );
        return Vec::new();
    }

    let mut imports = Vec::new();
    for item in &parsed.program.body {
        match item {
            oxc_ast::ast::Statement::ImportDeclaration(decl) => {
                imports.push(decl.source.value.to_string());
            }
            oxc_ast::ast::Statement::ExportNamedDeclaration(decl) => {
                if let Some(source) = &decl.source {
                    imports.push(source.value.to_string());
                }
            }
            oxc_ast::ast::Statement::ExportAllDeclaration(decl) => {
                imports.push(decl.source.value.to_string());
            }
            _ => {}
        }
    }
    imports
}

fn is_native_module(name: &str, native_names: &[&str]) -> bool {
    native_names.contains(&name)
}

fn virtual_name(abs_path: &str, entry_dir: &str) -> String {
    abs_path
        .strip_prefix(entry_dir)
        .unwrap_or(abs_path)
        .trim_start_matches('/')
        .trim_start_matches('\\')
        .to_string()
}

pub fn bundle(entry: &Path, resolver: &Resolver, native_names: &[&str]) -> Result<Bundle, String> {
    let entry_abs = std::fs::canonicalize(entry)
        .map_err(|e| format!("cannot resolve entry {}: {}", entry.display(), e))?;
    let entry_abs_str = entry_abs.to_string_lossy().replace('\\', "/");
    let entry_dir = entry_abs
        .parent()
        .ok_or_else(|| "entry has no parent dir".to_string())?
        .to_string_lossy()
        .replace('\\', "/");

    let mut modules: HashMap<String, String> = HashMap::new();
    let mut original_paths: HashMap<String, String> = HashMap::new();
    let mut visited: HashSet<String> = HashSet::new();
    let mut queue: VecDeque<(String, String)> = VecDeque::new();

    let entry_vname = "main".to_string();
    queue.push_back((entry_abs_str.clone(), entry_vname));

    while let Some((abs_path, vname)) = queue.pop_front() {
        let normalized = abs_path.replace('\\', "/");
        if visited.contains(&normalized) {
            continue;
        }
        visited.insert(normalized.clone());

        let source = std::fs::read_to_string(&abs_path)
            .map_err(|e| format!("cannot read {}: {}", abs_path, e))?;

        let file_ext = Path::new(&abs_path)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");

        let code = if file_ext == "json" {
            serde_json::from_str::<serde_json::Value>(&source)
                .map_err(|e| format!("invalid JSON {}: {}", abs_path, e))?;
            format!("export default {source};")
        } else if file_ext == "txt" || file_ext == "text" {
            let json_str = serde_json::to_string(&source)
                .map_err(|e| format!("cannot escape text {}: {}", abs_path, e))?;
            format!("export default {json_str};")
        } else if typescript::is_ts_ext(&abs_path) {
            typescript::strip_types_fast_default(&source)?
        } else {
            source
        };

        let imports = extract_imports(&code, &abs_path);

        for specifier in &imports {
            if is_native_module(specifier, native_names) {
                continue;
            }
            match resolver.resolve_impl(&abs_path, specifier) {
                Ok(resolved) => {
                    let resolved_norm = resolved.replace('\\', "/");
                    if !visited.contains(&resolved_norm) {
                        let dep_vname = virtual_name(&resolved_norm, &entry_dir);
                        queue.push_back((resolved, dep_vname));
                    }
                }
                Err(e) => {
                    return Err(format!(
                        "cannot resolve '{}' from '{}': {}",
                        specifier, abs_path, e
                    ));
                }
            }
        }

        modules.insert(vname.clone(), code);
        original_paths.insert(vname, abs_path);
    }

    Ok(Bundle {
        entry: "main".to_string(),
        modules,
        original_paths,
    })
}
