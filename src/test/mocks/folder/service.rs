use crate::services::folder::FolderService;
use crate::entities::models::Folder;
use crate::entities::builders::{ Builder, FolderBuilder };
use crate::services::folder::CreateRequest;
use crate::services::folder::UpdateRequest;
use crate::services::error::ServiceError;

pub struct FolderServiceMock;

impl FolderServiceMock {
    pub fn new() -> Self {
        Self
    }
}

impl FolderService for FolderServiceMock {
    fn all(&self, user_id: i32) -> Result<Vec<Folder>, ServiceError> {
        let folders = vec![
            factory!(Folder, user_id, None),
            factory!(Folder, user_id, None),
            factory!(Folder, user_id, None),
            factory!(Folder, user_id, None),
            factory!(Folder, user_id, None),
        ];

        Ok(folders)
    }

    fn find(&self, folder_id: i32) -> Result<Folder, ServiceError> {
        let mut folder = factory!(Folder, 1, None);

        folder.set_id(folder_id);

        Ok(folder)
    }

    fn create(&self, request: CreateRequest) -> Result<Folder, ServiceError> {
        let mut folder = factory!(Folder, request.user_id, request.parent_id);

        folder.set_name(request.name);

        Ok(folder)
    }

    fn update(&self, request: UpdateRequest) -> Result<Folder, ServiceError> {
        let mut folder = factory!(Folder, request.user_id, request.parent_id);

        folder.set_id(request.id);
        folder.set_name(request.name);

        Ok(folder)
    }

    fn delete(&self, id: i32) -> Result<Folder, ServiceError> {
        let mut folder = factory!(Folder, 1, None);

        folder.set_id(id);

        Ok(folder)
    }
}
