use std::path::Path;

use blirssa::Library;
use config::{BuildConfig, BuildOutput, BuildProfile};
use inkwell::{context::Context, targets::{Target, InitializationConfig, TargetTriple, RelocMode, CodeModel, FileType, TargetMachine}, OptimizationLevel};
use lower_blirssa::lower_blirssa_library;

pub mod config;

pub fn compile(library: Library, config: BuildConfig) {
    let context = Context::create();

    let output_file = format!("bin/lib{}", library.name());

    let module = lower_blirssa_library(library, &context).unwrap();

    if config.output == BuildOutput::LLVM {
        let _ = module
            .print_to_file(format!("{output_file}.ll"));
        return;
    }

    Target::initialize_x86(&InitializationConfig::default());

    let optimization_level = match config.profile {
        BuildProfile::Debug => OptimizationLevel::Less,
        BuildProfile::Release => OptimizationLevel::Aggressive,
    };

    let target_triple = match config.target {
        Some(triple) => TargetTriple::create(&triple),
        None => TargetMachine::get_default_triple(),
    };

    let target = Target::from_triple(&target_triple).unwrap();

    let target_machine = target
        .create_target_machine(&target_triple,
            TargetMachine::get_host_cpu_name().to_str().unwrap(),
            "+avx2",
            optimization_level,
            RelocMode::Static,
            CodeModel::Default)
        .unwrap();

    let file_type = match config.output {
        BuildOutput::ASM => FileType::Assembly,
        BuildOutput::Object => FileType::Object,
        _ => panic!(),
    };

    let file_name = match config.output {
        BuildOutput::ASM => format!("{output_file}.s"),
        BuildOutput::Object => format!("{output_file}.o"),
        _ => panic!()
    };

    let _ = target_machine.write_to_file(&module, file_type, Path::new(&file_name));
}