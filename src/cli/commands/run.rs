use clap::Args;
use std::{
    fs,
    io::{self, ErrorKind},
    path::{self, Path, PathBuf},
};

use crate::{DIRECTORY_INDEX_MODULE_NAME, EvalResult, Interpreter, RALIX_VALID_EXTENSIONS};

#[derive(Args, Debug)]
pub struct RunArguments {
    file: Option<PathBuf>,
    #[arg(long)] skip_type_checking: bool,
    #[arg(long)] print_ast: bool,
}

pub fn run(args: RunArguments) -> io::Result<()> {
    match &args.file {
        Some(path) => run_module(path, &args),
        None => run_project(),
    }
}

fn run_module(module_path: &Path, args: &RunArguments) -> io::Result<()> {
    let (module_file_path, working_directory) = resolve_module_file(module_path)?;

    let mut interpreter = Interpreter::new(working_directory)?;
    let module_source = fs::read_to_string(module_file_path)?;

    let program = match interpreter.parse(&module_source) {
        Ok(program) => {
            if args.print_ast {
                let program_as_json_string = serde_json::to_string_pretty(&program)?;
                println!("{program_as_json_string}");
                return Ok(());
            } else {
                program
            }
        }
        Err(parse_error) => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                parse_error.to_string(),
            ));
        }
    };

    if !args.skip_type_checking 
        && let Err(check_error) = interpreter.check(&program) 
    {
        return Err(io::Error::new(
            ErrorKind::InvalidInput,
            check_error.to_string(),
        ));
    }
    
    match interpreter.execute(program) {
        EvalResult::Value(_) | EvalResult::Return(_) | EvalResult::NoValue => Ok(()),
        EvalResult::Err(final_runtime_error) => Err(io::Error::new(ErrorKind::InvalidData, final_runtime_error.to_string())),
    }
}

fn run_project() -> io::Result<()> {
    todo!(
        "Project folder implemention is coming soon, for now please specify the module you want to execute"
    )
}

/// For first return type, basically converts "module/submodule" to
/// ->  "module/submodule.(rl | rlx | ralix)" or
/// ->  "module/submodule/package.(rl | rlx | ralix)"
/// if one this paths were found
///
/// For second return type it gets the parent directory if it's not a dir.
/// Itself otherwise
fn resolve_module_file(module_path: &Path) -> io::Result<(PathBuf, PathBuf)> {
    let mut found = false;
    let absolute = path::absolute(module_path)?;
    let mut working_directory = absolute.clone();
    let mut module_path = module_path.to_path_buf();

    if absolute.is_dir() {
        for ext in RALIX_VALID_EXTENSIONS {
            let directory_index_module_name = DIRECTORY_INDEX_MODULE_NAME.to_string() + *ext;
            let dir_index_module_file = absolute.join(&directory_index_module_name);
            if fs::exists(&dir_index_module_file)? {
                module_path = dir_index_module_file;
                found = true;
                break;
            }
        }
    } else {
        working_directory = working_directory.parent().unwrap().to_path_buf();

        for ext in RALIX_VALID_EXTENSIONS {
            let ext_without_dot /* ".rl" -> "rl" */ = &ext[1..];
            let file_module_path = absolute.clone();

            if let Some(file_module_extention) = file_module_path
                .extension()
                .map(|m_ext| m_ext.to_string_lossy())
            {
                if fs::exists(&file_module_path)? && ext_without_dot == &*file_module_extention {
                    found = true;
                    module_path = file_module_path;
                    break;
                }
            } else {
                let module_path_with_extention = {
                    let mut p = file_module_path.clone();
                    p.set_extension(ext_without_dot);
                    p
                };
                if fs::exists(&module_path_with_extention)? {
                    found = true;
                    module_path = module_path_with_extention;
                    break;
                }
            }
        }
    }

    if found {
        Ok((module_path, working_directory))
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("A valid ralix module not found for {module_path:?}"),
        ))
    }
}
