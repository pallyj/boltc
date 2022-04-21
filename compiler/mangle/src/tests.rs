use crate::{defs::{MangledFunction, MangledStruct},
            MangledType, Path};

#[test]
fn test_struct() {
    let int_path = Path::new("lang").append("Int");
    let file_error_path = Path::new("std").append("File").append("Error");

    assert_eq!("4lang3IntS", MangledStruct(&int_path).to_string());
    assert_eq!("3std4File5ErrorS",
               MangledStruct(&file_error_path).to_string());
}

#[test]
fn test_function() {
    let open_path = Path::new("std").append("File").append("open");

    let file_path = Path::new("std").append("File");
    let file_type = MangledType::Struct(file_path);

    let args = vec![MangledType::StringSlice, MangledType::Integer1, file_type];

    let mangled = MangledFunction { path: &open_path,
                                    args,
                                    labels: vec![Some("path"), None, None] };

    assert_eq!("3std4File4openFrb3std4FileSE4path00", mangled.to_string());
}
