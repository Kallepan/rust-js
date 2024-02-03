use std::sync::Arc;
use swc::config::{Config, JscConfig, Options};
use swc::{try_with_handler, Compiler, HandlerOpts, TransformOutput};
use swc_common::sync::Lazy;
use swc_common::{FileName, FilePathMapping, SourceMap};
use swc_ecma_ast::EsVersion;
use swc_ecma_parser::{Syntax, TsConfig};

static SWC_GLOBALS: Lazy<swc_common::Globals> = Lazy::new(swc_common::Globals::new);

static COMPILER: Lazy<Arc<Compiler>> = Lazy::new(|| {
    let cm = Arc::new(SourceMap::new(FilePathMapping::empty()));

    Arc::new(Compiler::new(cm.clone()))
});

fn get_compiler() -> Arc<Compiler> {
    COMPILER.clone()
}

pub fn transform(filename: String, src: String) -> TransformOutput {
    let handler_opts = HandlerOpts {
        ..Default::default()
    };
    let c = get_compiler();

    swc_common::GLOBALS.set(&SWC_GLOBALS, || {
        // Your code here...
        try_with_handler(c.cm.clone(), handler_opts, |handler| {
            c.process_js_file(
                c.cm.new_source_file(FileName::Anon, src),
                handler,
                &Options {
                    config: Config {
                        jsc: JscConfig {
                            syntax: Some(Syntax::Typescript(TsConfig {
                                decorators: true,
                                tsx: true,
                                ..Default::default()
                            })),
                            target: Some(EsVersion::Es2020),
                            ..Default::default()
                        },
                        minify: Into::into(false),
                        ..Default::default()
                    },
                    filename,
                    swcrc: false,
                    ..Default::default()
                },
            )
        })
        .unwrap()
    })
}
