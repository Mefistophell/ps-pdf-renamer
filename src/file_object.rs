use std::path::PathBuf;

pub struct FileObject<'a> {
    file_name: &'a str,
    file_ext: Option<&'a str>,
    dir_name: &'a str,
}

impl<'a> FileObject<'a> {
    pub fn new(dir_name: &'a str, file_name: &'a str, file_ext: Option<&'a str>) -> Self {
        FileObject {
            file_name,
            file_ext,
            dir_name,
        }
    }

    pub fn get_file_path(&self) -> PathBuf {
        let file_path = match self.file_ext {
            Some(ext) => {
                format!("{}/{}.{}", self.dir_name, self.file_name, ext)
            }
            None => {
                format!("{}/{}", self.dir_name, self.file_name)
            }
        };

        PathBuf::from(file_path)
    }
}
