use std::{
    fs,
    process::{Command, Stdio},
};

pub(crate) fn man2html(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let man2html_output = if path.trim_end().ends_with(".gz") {
        let zcat_output = Command::new("zcat")
            .arg(path)
            .stdout(Stdio::piped())
            .spawn()?
            .stdout
            .ok_or_else(|| "zcat failed")?;

        Command::new("man2html").stdin(zcat_output).output()?
    } else {
        Command::new("man2html").arg(path).output()?
    };

    let file_raw = String::from_utf8(man2html_output.stdout)?;
    match file_raw.find("<HTML") {
        Some(n) => Ok(file_raw[n..].to_string()),
        None => Ok(file_raw),
    }
}

pub(crate) fn manpath() -> Vec<String> {
    let output = Command::new("manpath")
        .output()
        .expect("Starting `manpath` failed. Is `manpath` installed?");

    String::from_utf8(output.stdout)
        .expect("Parsing manpath failed!")
        .trim_end()
        .split(':')
        .map(|str| str.to_owned())
        .collect()
}

pub(crate) fn manpages(paths: &Vec<String>) -> Vec<String> {
    let mut pages: Vec<String> = vec![];

    for path in paths {
        let children = fs::read_dir(path).expect("Reading directory failed!");
        for child in children {
            let grandchildren =
                fs::read_dir(child.unwrap().path()).expect("Reading child dir failed!");
            for grandchild in grandchildren {
                let path = grandchild.unwrap().path();
                pages.push(path.to_str().unwrap().to_owned());
            }
        }
    }
    pages
}
