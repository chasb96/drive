#[cfg(not(test))]
macro_rules! resolve {
    (FileService) => { crate::services::file::implementation::Service::new() };

    (FolderService) => { crate::services::folder::implementation::Service };

    (UserService) => { crate::services::user::implementation::Service };

    (StorageService) => { crate::services::StorageService };

    (FileController) => { crate::controllers::FileController::new(resolve!(FileService)) };

    (FolderController) => { crate::controllers::FolderController };

    (UserController) => { crate::controllers::UserController };
}

#[cfg(test)]
macro_rules! resolve {
    (FileService) => { crate::services::file::FileServiceMock::new() };

    (FolderService) => { crate::services::folder::implementation::Service };

    (UserService) => { crate::services::user::implementation::Service };

    (StorageService) => { crate::services::StorageService };

    (FileController) => { crate::controllers::FileController::new(resolve!(FileService)) };

    (FolderController) => { crate::controllers::FolderController };

    (UserController) => { crate::controllers::UserController };
}
