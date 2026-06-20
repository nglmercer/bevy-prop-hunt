use bevy::asset::HandleTemplate;
use bevy::ecs::template::TemplateContext;
use bevy::prelude::*;

#[derive(Default)]
pub struct BuildMesh3d(pub HandleTemplate<Mesh>);

impl From<HandleTemplate<Mesh>> for BuildMesh3d {
    fn from(value: HandleTemplate<Mesh>) -> Self {
        Self(value)
    }
}

impl Template for BuildMesh3d {
    type Output = Mesh3d;

    fn build_template(&self, context: &mut TemplateContext) -> Result<Self::Output> {
        Ok(Mesh3d(self.0.build_template(context)?))
    }

    fn clone_template(&self) -> Self {
        BuildMesh3d(self.0.clone_template())
    }
}
