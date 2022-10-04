use bevy::prelude::*;
use bevy::render::texture::DEFAULT_IMAGE_HANDLE;

#[derive(Component, Debug, Default, Clone, Reflect, Deref, DerefMut)]
#[repr(C)]
pub struct IndependentSprite(pub Sprite);

#[derive(Component, Debug, Default, Clone, Reflect, Deref, DerefMut)]
pub struct IndependentTextureAtlasSprite(pub TextureAtlasSprite);

#[derive(Copy, Component, Default, Debug, Clone, Reflect, Deref, DerefMut)]
pub struct IndependentTransform(pub Transform);

#[derive(Copy, Component, Debug, Default, Clone, Reflect, Deref, DerefMut)]
pub struct ComputedTransform(pub GlobalTransform);

#[derive(Bundle, Clone)]
pub struct IndependentSpriteBundle {
    pub sprite: IndependentSprite,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub texture: Handle<Image>,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
    pub independent_transform: IndependentTransform,
    pub computed_transform: ComputedTransform,
}

impl Default for IndependentSpriteBundle {
    fn default() -> Self {
        Self {
            sprite: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
            texture: DEFAULT_IMAGE_HANDLE.typed(),
            visibility: Default::default(),
            computed_visibility: Default::default(),    
            independent_transform: Default::default(),
            computed_transform: Default::default(),
        }
    }
}

#[derive(Bundle, Clone)]
pub struct IndependentSpriteSpriteBundle {
    pub sprite: IndependentTextureAtlasSprite,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub texture_atlas: Handle<TextureAtlas>,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
    pub independent_transform: IndependentTransform,
    pub computed_transform: ComputedTransform,
}

impl Default for IndependentSpriteSpriteBundle {
    fn default() -> Self {
        Self {
            sprite: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
            texture_atlas: Default::default(),
            visibility: Default::default(),
            computed_visibility: Default::default(),    
            independent_transform: Default::default(),
            computed_transform: Default::default(),
        }
    }
}

fn compute_transform(
    mut query: Query<(&mut ComputedTransform, &IndependentTransform, &GlobalTransform), Or<(Changed<GlobalTransform>, Changed<IndependentTransform>)>>,
) {
    query.for_each_mut(|(mut computed, independent, global)| {
        *computed = ComputedTransform(independent.compute_affine().into());
        *computed.translation_mut() += global.translation_vec3a();
    });
}

pub struct IndependentSpritePlugin;

impl Plugin for IndependentSpritePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system_to_stage(
            CoreStage::PostUpdate, 
            compute_transform
            .after(bevy::transform::TransformSystem::TransformPropagate)
        );
    }
}