mod file;
mod folder;
mod user;

pub use self::file::FileBuilder;
pub use self::folder::FolderBuilder;
pub use self::user::UserBuilder;

pub trait Builder {
    type Out;

    fn build(self) -> Self::Out;
}
