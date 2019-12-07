mod file;
mod folder;
mod user;

pub use self::file::File;
pub use self::folder::Folder;
pub use self::user::User;

pub trait Model: Clone + Sized {}
