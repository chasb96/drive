#[allow(unused_macros)]
macro_rules! factory {
    {User} => (
        UserBuilder::new()
            .with_id(fake!(Number.digit).parse::<i32>().unwrap_or(0))
            .with_name(fake!(Name.name))
            .with_email(fake!(Internet.safe_email))
            .with_password(fake!(Lorem.word).to_string())
            .build()
    );

    {Folder, $user_id:expr, $parent_id:expr} => (
        FolderBuilder::new()
            .with_id(fake!(Number.digit).parse::<i32>().unwrap_or(0))
            .with_name(fake!(Lorem.words(3)).join(" "))
            .with_parent_id($parent_id)
            .with_user_id($user_id)
            .build()
    );

    {File, $folder_id:expr} => {
        FileBuilder::new()
            .with_id(fake!(Number.digit).parse::<i32>().unwrap_or(0))
            .with_name(fake!(Lorem.words(3)).join(" "))
            .with_folder_id($folder_id)
            .with_file_name(fake!(Lorem.words(10)).join(""))
            .with_extension(fake!(Lorem.word).to_string())
            .build()
    }
}
