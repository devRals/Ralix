use std::{
    fs, io,
    path::{self, Path, PathBuf},
};

use serde::Serialize;

use crate::{
    DIRECTORY_INDEX_MODULE_NAME, Expression, RALIX_VALID_EXTENSIONS, expressions::Identifier,
    types::Type,
};

#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct Binding {
    pub ident: Identifier,
    pub type_annotation: Option<Type>,
    pub value: Expression,
    pub is_constant: bool,
}

pub fn resolve_file_module_path<P: AsRef<Path>>(
    working_directory_path: P,
    module_names: &[Identifier],
) -> io::Result<PathBuf> {
    let relative_path = PathBuf::from(&module_names.join(path::MAIN_SEPARATOR_STR));
    let mut path = {
        let wd = working_directory_path.as_ref();
        wd.join(relative_path)
    };
    let mut found = false;

    if path.is_dir() {
        for ext in RALIX_VALID_EXTENSIONS {
            let directory_index_file_module_name = DIRECTORY_INDEX_MODULE_NAME.to_string() + ext;
            if fs::exists(path.join(&directory_index_file_module_name))? {
                found = true;
                path.push(directory_index_file_module_name);
                break;
            }
        }
    } else {
        for ext in RALIX_VALID_EXTENSIONS {
            let file_module_name = path.to_string_lossy().to_string() + ext;
            if fs::exists(&file_module_name)? {
                path = PathBuf::from(file_module_name);
                found = true;
                break;
            }
        }
    }

    if found {
        Ok(path)
    } else {
        Err(io::Error::new(
            io::ErrorKind::InvalidFilename,
            format!(
                "{} is an invalid path. Valid module extensions are {}",
                path.to_string_lossy(),
                RALIX_VALID_EXTENSIONS.join(", ")
            ),
        ))
    }
}
