use crate::entities::builders::Builder;
use crate::entities::models::User;

pub struct UserBuilder {
    id: i32,
    name: String,
    email: String,
    password: String,
    root: Option<i32>,
    role: String,
}

impl Default for UserBuilder {
    fn default() -> Self {
        Self {
            id: i32::default(),
            name: String::default(),
            email: String::default(),
            password: String::default(),
            root: None,
            role: "user".to_string(),
        }
    }
}

#[allow(dead_code)]
impl UserBuilder {
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

    pub fn with_email(mut self, email: String) -> Self {
        self.email = email;

        self
    }

    pub fn with_password(mut self, password: String) -> Self {
        self.password = password;

        self
    }

    pub fn with_root(mut self, root: Option<i32>) -> Self {
        self.root = root;

        self
    }

    pub fn with_role(mut self, role: String) -> Self {
        self.role = role;

        self
    }
}

impl Builder for UserBuilder {
    type Out = User;

    fn build(self) -> Self::Out {
        let mut user = User::new();

        user.set_id(self.id);
        user.set_name(self.name);
        user.set_email(self.email);
        user.set_password(self.password);
        user.set_root(self.root);
        user.set_role(self.role);

        user
    }
}
