macro_rules! resolve {
    (FileService) => {
        crate::services::file::service::Service::new(resolve!(FileStore))
    };

    (FolderService) => {
        crate::services::folder::service::Service::new(
            resolve!(FolderStore),
            resolve!(FileService),
        )
    };

    (UserService) => {
        crate::services::user::service::Service::new(
            resolve!(UserStore),
            resolve!(FolderService),
        )
    };

    (StorageService) => {
        crate::services::storage::implementation::Service::new(
            resolve!(StorageDriver)
        )
    };

    (FileController) => {
        crate::controllers::file::controller::Controller::new(
            resolve!(FileService),
            resolve!(StorageService),
            resolve!(FileAuthorizer),
        )
    };

    (FolderController) => {
        crate::controllers::folder::controller::Controller::new(
            resolve!(FolderService),
            resolve!(FolderAuthorizer)
        )
    };

    (UserController) => {
        crate::controllers::user::controller::Controller::new(
            resolve!(UserService),
            resolve!(UserAuthorizer)
        )
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

    (FileAuthorizer) => {
        crate::policies::file::Authorizer::new()
    };

    (FolderAuthorizer) => {
        crate::policies::folder::Authorizer::new()
    };

    (UserAuthorizer) => {
        crate::policies::user::Authorizer::new()
    };

    (StorageDriver) => {
        crate::env::Env::storage_driver()
    };
}
