use services::folder::FolderService;
use crate::entities::models::Folder;
use diesel::result::Error;
use crate::entities::builders::{ Builder, FolderBuilder };

pub struct FolderServiceMock;

impl FolderServiceMock {
    pub fn new() -> Self {
        Self
    }
}

impl FolderService for FolderServiceMock {
    fn all(&self, user_id: i32) -> Result<Vec<Folder>, Error> {
        let folders = vec![
            factory!(Folder, user_id, None),
            factory!(Folder, user_id, None),
            factory!(Folder, user_id, None),
            factory!(Folder, user_id, None),
            factory!(Folder, user_id, None),
        ];

        Ok(folders)
    }

    fn find(&self, folder_id: i32) -> Result<Folder, Error> {
        let mut folder = factory!(Folder, 1, None);

        folder.set_id(folder_id);

        Ok(folder)
    }

    fn create(
        &self,
        name: String,
        user_id: i32,
        parent_id: Option<i32>
    ) -> Result<Folder, Error> {
        let mut folder = factory!(Folder, user_id, parent_id);

        folder.set_name(name);

        Ok(folder)
    }

    fn update(
        &self,
        id: i32,
        name: String,
        user_id: i32,
        parent_id: Option<i32>
    ) -> Result<Folder, Error> {
        let mut folder = factory!(Folder, user_id, parent_id);

        folder.set_id(id);
        folder.set_name(name);

        Ok(folder)
    }

    fn delete(&self, id: i32) -> Result<Folder, Error> {
        let mut folder = factory!(Folder, 1, None);

        folder.set_id(id);

        Ok(folder)
    }
}
