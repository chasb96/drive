macro_rules! resolve {
    (FileService) => {
        crate::services::file::implementation::Service::new(resolve!(FileStore))
    };

    (FolderService) => {
        crate::services::folder::implementation::Service::new(
            resolve!(FolderStore),
            resolve!(FileService),
        )
    };

    (UserService) => {
        crate::services::user::implementation::Service::new(
            resolve!(UserStore),
            resolve!(FolderService),
        )
    };

    (StorageService) => {
        crate::services::storage::implementation::Service::new()
    };

    (FileController) => {
        crate::controllers::file::implementation::Controller::new(
            resolve!(FileService),
            resolve!(StorageService),
        )
    };

    (FolderController) => {
        crate::controllers::FolderController
    };

    (UserController) => {
        crate::controllers::UserController
    };

    (UserStore) => {
        crate::entities::diesel::stores::user::Store::new()
    };

    (FolderStore) => {
        crate::entities::diesel::stores::folder::Store::new()
    };

    (FileStore) => {
        crate::entities::diesel::stores::file::Store::new()
    };
}
