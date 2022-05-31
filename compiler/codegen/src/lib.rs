use std::path::Path;

use blirssa::Library;
use config::{BuildConfig, BuildOutput, BuildProfile};
use inkwell::{context::Context,
              module::Module,
              passes::{PassManager, PassManagerBuilder},
              targets::{CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine, TargetTriple},
              OptimizationLevel};
use lower_blirssa::lower_blirssa_library;

pub mod config;

pub fn compile(library: Library, config: BuildConfig) {
    let context = Context::create();

    let output_file = format!("bin/lib{}", library.name());

    let module = lower_blirssa_library(library, &context).unwrap();

    let pass_manager = build_pass_manager(config.profile);

    pass_manager.run_on(&module);

    if config.output == BuildOutput::LLVM {
        let _ = module.print_to_file(format!("{output_file}.ll"));
        return;
    }

    Target::initialize_x86(&InitializationConfig::default());

    let optimization_level = match config.profile {
        BuildProfile::Debug => OptimizationLevel::Less,
        BuildProfile::Less => OptimizationLevel::Less,
        BuildProfile::Normal => OptimizationLevel::Default,
        BuildProfile::Aggressive => OptimizationLevel::Aggressive,
    };

    let target_triple = match config.target {
        Some(triple) => TargetTriple::create(&triple),
        None => TargetMachine::get_default_triple(),
    };

    let target = Target::from_triple(&target_triple).unwrap();

    let target_machine = target.create_target_machine(&target_triple,
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
        _ => panic!(),
    };

    let _ = target_machine.write_to_file(&module, file_type, Path::new(&file_name));
}

pub fn build_pass_manager<'a>(profile: BuildProfile) -> PassManager<Module<'a>> {
    let builder = PassManagerBuilder::create();

    let optimization_level = match profile {
        BuildProfile::Debug => OptimizationLevel::Less,
        BuildProfile::Less => OptimizationLevel::Less,
        BuildProfile::Normal => OptimizationLevel::Default,
        BuildProfile::Aggressive => OptimizationLevel::Aggressive,
    };

    builder.set_optimization_level(optimization_level);

    let pass_manager: PassManager<Module> = PassManager::create(());

    pass_manager.add_constant_propagation_pass();
    pass_manager.add_constant_merge_pass();

    builder.populate_module_pass_manager(&pass_manager);

    pass_manager.add_function_inlining_pass();
    pass_manager.add_tail_call_elimination_pass();
    pass_manager.add_loop_rotate_pass();
    pass_manager.add_loop_unroll_pass();

    pass_manager
}
