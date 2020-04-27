use neon::prelude::*;
use std::fs;
use std::io;
use std::path::Path;

//
// exported function.
//
fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    let dir = Path::new("c:/home/repos/dotemacs/");
    let mut s = String::new();

    match parse_dir(dir) {
        Ok(vec) => {
            for file in vec {
                println!("file name: {}", file);
		s.push_str(&file);
		s.push_str("\n");
            }
            println!("added directory {:?}", dir)
        }
        Err(err) => println!("Error: {}", err),
    }

    Ok(cx.string(&s))
}

register_module!(mut cx, {
    cx.export_function("hello", hello)
});

//
// Recursively parsed 'dir' and returns a vector of all the files.
//
fn parse_dir(dir: &Path) -> io::Result<Vec<String>> {
    let mut vec = Vec::new();
    parse_dir_helper(dir, &mut vec)?;

    Ok(vec)
}

//
// Help function for parse_dir.
//
fn parse_dir_helper(dir: &Path, vec: &mut Vec<String>) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();

        if path.is_file() {
            if let Some(name) = path.file_name().unwrap().to_str() {
                vec.push(String::from(name));
            } else {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "file name contains non-utf8 characters",
                ));
            }
        } else if path.is_dir() {
	    parse_dir_helper(&path, vec)?;
        }
    }

    Ok(())
}
