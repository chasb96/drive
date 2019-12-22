use crate::entities::builders::Builder;
use crate::entities::models::File;

pub struct FileBuilder {
    id: i32,
    name: String,
    file_name: String,
    folder_id: i32,
    extension: String,
    public: bool,
}

impl Default for FileBuilder {
    fn default() -> Self {
        Self {
            id: 0,
            name: String::default(),
            file_name: String::default(),
            folder_id: 0,
            extension: String::default(),
            public: false,
        }
    }
}

#[allow(dead_code)]
impl FileBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_id(mut self, id: i32) -> Self {
        self.id = id;

        self
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.name = name;

        self
    }

    pub fn with_folder_id(mut self, folder_id: i32) -> Self {
        self.folder_id = folder_id;

        self
    }

    pub fn with_file_name(mut self, file_name: String) -> Self {
        self.file_name = file_name;

        self
    }

    pub fn with_extension(mut self, extension: String) -> Self {
        self.extension = extension;

        self
    }

    pub fn with_public(mut self, public: bool) -> Self {
        self.public = public;

        self
    }
}

impl Builder for FileBuilder {
    type Out = File;

    fn build(self) -> Self::Out {
        let mut file = File::new();

        file.set_id(self.id);
        file.set_name(self.name);
        file.set_folder_id(self.folder_id);
        file.set_extension(self.extension);
        file.set_public(self.public);
        file.set_file_name(self.file_name);

        file
    }
}
