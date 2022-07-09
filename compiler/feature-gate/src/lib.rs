/*
features:
    - arrow_function
    - var_declare
    - let_mut_declare
    - let_var_declar
    - param_label_consolidate
    - repeat_loop
    - while_loop
    - while_let_loop
    - if_let
    - guard
    - guard_let
*/

use std::sync::atomic::{AtomicBool, Ordering};

use colored::Colorize;

static ARROW_FUNCTION: AtomicBool = AtomicBool::new(true);
static VAR_DECLARE: AtomicBool = AtomicBool::new(true);
static LET_MUT_DECLARE: AtomicBool = AtomicBool::new(false);
static LET_VAR_DECLAR: AtomicBool = AtomicBool::new(true);
static PARAM_LABEL_CONSOLIDATE: AtomicBool = AtomicBool::new(true);
static REPEAT_LOOP: AtomicBool = AtomicBool::new(true);
static WHILE_LOOP: AtomicBool = AtomicBool::new(true);
static WHILE_LET_LOOP: AtomicBool = AtomicBool::new(true);
static IF_LET: AtomicBool = AtomicBool::new(true);
static GUARD: AtomicBool = AtomicBool::new(true);
static GUARD_LET: AtomicBool = AtomicBool::new(true);

pub fn has_feature(name: &'static str) -> bool {
    match name {
        "arrow_function" => ARROW_FUNCTION.load(Ordering::Relaxed),
        "var_declare" => VAR_DECLARE.load(Ordering::Relaxed),
        "let_mut_declare" => LET_MUT_DECLARE.load(Ordering::Relaxed),
        "let_var_declare" => LET_VAR_DECLAR.load(Ordering::Relaxed),
        "param_label_consolidate" => PARAM_LABEL_CONSOLIDATE.load(Ordering::Relaxed),
        "repeat_loops" => REPEAT_LOOP.load(Ordering::Relaxed),
        "while_loops" => WHILE_LOOP.load(Ordering::Relaxed),
        "while_let_loops" => WHILE_LET_LOOP.load(Ordering::Relaxed),
        "if_let" => IF_LET.load(Ordering::Relaxed),
        "guard" => GUARD.load(Ordering::Relaxed),
        "guard_let" => GUARD_LET.load(Ordering::Relaxed),
        _ => false
    }
}

pub fn enable_feature(name: &str) {
    match name {
        "arrow_function" => ARROW_FUNCTION.store(true, Ordering::Relaxed),
        "var_declare" => VAR_DECLARE.store(true, Ordering::Relaxed),
        "let_mut_declare" => LET_MUT_DECLARE.store(true, Ordering::Relaxed),
        "let_var_declare" => LET_VAR_DECLAR.store(true, Ordering::Relaxed),
        "param_label_consolidate" => PARAM_LABEL_CONSOLIDATE.store(true, Ordering::Relaxed),
        "repeat_loops" => REPEAT_LOOP.store(true, Ordering::Relaxed),
        "while_loops" => WHILE_LOOP.store(true, Ordering::Relaxed),
        "while_let_loops" => WHILE_LET_LOOP.store(true, Ordering::Relaxed),
        "if_let" => IF_LET.store(true, Ordering::Relaxed),
        "guard" => GUARD.store(true, Ordering::Relaxed),
        "guard_let" => GUARD_LET.store(true, Ordering::Relaxed),
        _ => { println!("{} feature {} is not recognized", "error:".red().bold(), name.bold()); return }
    }

    println!("{} feature {} is unstable", "warning:".yellow().bold(), name.bold());
}