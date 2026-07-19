use bevy::asset::{AssetPath, HandleTemplate};
use bevy::prelude::*;

pub enum AssetRef<T: Asset> {
    /// Creates a [`Handle`] by calling [`AssetServer::load`] on the given [`AssetPath`].
    Path(AssetPath<'static>),
    /// Creates a [`Handle`] by cloning the given [`Handle`] value.
    Handle(Handle<T>),
}

impl<T: Asset> Clone for AssetRef<T> {
    fn clone(&self) -> Self {
        match self {
            Self::Path(asset_path) => Self::Path(asset_path.clone()),
            Self::Handle(handle) => Self::Handle(handle.clone()),
        }
    }
}

impl<T: Asset> From<Handle<T>> for AssetRef<T> {
    fn from(value: Handle<T>) -> Self {
        Self::Handle(value)
    }
}

impl<T: Asset> From<AssetPath<'static>> for AssetRef<T> {
    fn from(value: AssetPath<'static>) -> Self {
        Self::Path(value)
    }
}

impl<T: Asset> FromTemplate for AssetRef<T> {
    type Template = AssetRefTemplate<T>;
}

pub struct AssetRefTemplate<T: Asset>(pub HandleTemplate<T>);

impl<T: Asset> Default for AssetRefTemplate<T> {
    fn default() -> Self {
        Self(HandleTemplate::default())
    }
}

impl<T: Asset> From<HandleTemplate<T>> for AssetRefTemplate<T> {
    fn from(value: HandleTemplate<T>) -> Self {
        Self(value)
    }
}

impl<T: Asset> Template for AssetRefTemplate<T> {
    type Output = AssetRef<T>;

    fn build_template(
        &self,
        context: &mut bevy::ecs::template::TemplateContext,
    ) -> Result<Self::Output> {
        match &self.0 {
            HandleTemplate::Path(asset_path) => Ok(AssetRef::Path(asset_path.clone())),
            HandleTemplate::Handle(handle) => Ok(AssetRef::Handle(handle.clone())),
            v @ HandleTemplate::Value(..) => Ok(AssetRef::Handle(v.build_template(context)?)),
        }
    }

    fn clone_template(&self) -> Self {
        Self(self.0.clone_template())
    }
}
