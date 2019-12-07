mod file;
mod folder;
mod user;

use entities::Entity;

pub trait Restrict<T: Restricted>: Entity {
    fn indexable_by(_restrict: &T) -> bool {
        true
    }

    fn creatable_by(_restrict: &T) -> bool {
        true
    }

    fn viewable_by(&self, _restrict: &T) -> bool {
        true
    }

    fn modifiable_by(&self, _restrict: &T) -> bool {
        true
    }

    fn deletable_by(&self, _restrict: &T) -> bool {
        true
    }
}

pub trait Restricted: Entity {
    fn can_index<T: Restrict<Self>>(&self) -> bool {
        T::indexable_by(&self)
    }

    fn can_create<T: Restrict<Self>>(&self) -> bool {
        T::creatable_by(&self)
    }

    fn can_view<T: Restrict<Self>>(&self, restricted: T) -> bool {
        restricted.viewable_by(self)
    }

    fn can_modify<T: Restrict<Self>>(&self, restricted: T) -> bool {
        restricted.modifiable_by(self)
    }

    fn can_delete<T: Restrict<Self>>(&self, restricted: T) -> bool {
        restricted.deletable_by(self)
    }
}
