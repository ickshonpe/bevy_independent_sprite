use bevy::prelude::*;
use bevy::render::Extract;
use bevy::render::RenderApp;
use bevy::render::RenderStage;
use bevy::render::texture::DEFAULT_IMAGE_HANDLE;
use bevy::sprite::ExtractedSprite;
use bevy::sprite::ExtractedSprites;
use bevy::sprite::SpriteSystem;
use copyless::VecHelper;

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


pub fn extract_independent_sprites(
    mut extracted_sprites: ResMut<ExtractedSprites>,
    texture_atlases: Extract<Res<Assets<TextureAtlas>>>,
    sprite_query: Extract<
        Query<(
            Entity,
            &ComputedVisibility,
            &IndependentSprite,
            &ComputedTransform,
            &Handle<Image>,
        )>,
    >,
    atlas_query: Extract<
        Query<(
            Entity,
            &ComputedVisibility,
            &IndependentTextureAtlasSprite,
            &ComputedTransform,
            &Handle<TextureAtlas>,
        )>,
    >,
) {
    let mut transform = GlobalTransform::default();
    for (entity, visibility, sprite, global_transform, handle) in sprite_query.iter() {
        if !visibility.is_visible() {
            continue;
        }
        *transform.translation_mut() = global_transform.translation_vec3a();
        extracted_sprites.sprites.alloc().init(ExtractedSprite {
            entity,
            color: sprite.color,
            transform,
            rect: None,
            custom_size: sprite.custom_size,
            flip_x: sprite.flip_x,
            flip_y: sprite.flip_y,
            image_handle_id: handle.id,
            anchor: sprite.anchor.as_vec(),
        });
    }
    for (entity, visibility, atlas_sprite, global_transform, texture_atlas_handle) in atlas_query.iter() {
        if !visibility.is_visible() {
            continue;
        }
        if let Some(texture_atlas) = texture_atlases.get(texture_atlas_handle) {
            let rect = Some(texture_atlas.textures[atlas_sprite.index as usize]);
            *transform.translation_mut() = global_transform.translation_vec3a();
            extracted_sprites.sprites.alloc().init(ExtractedSprite {
                entity,
                color: atlas_sprite.color,
                transform,
                rect,
                custom_size: atlas_sprite.custom_size,
                flip_x: atlas_sprite.flip_x,
                flip_y: atlas_sprite.flip_y,
                image_handle_id: texture_atlas.texture.id,
                anchor: atlas_sprite.anchor.as_vec(),
            });
        }
    }
}


pub struct IndependentSpritePlugin;

impl Plugin for IndependentSpritePlugin {
    fn build(&self, app: &mut App) {
        app
        .register_type::<IndependentSprite>()
        .register_type::<IndependentTextureAtlasSprite>()
        .register_type::<IndependentTransform>()
        .register_type::<ComputedTransform>()
        .add_system_to_stage(
            CoreStage::PostUpdate, 
            compute_transform
            .after(bevy::transform::TransformSystem::TransformPropagate)
        );

        if let Ok(render_app) = app.get_sub_app_mut(RenderApp) {
            render_app
            .add_system_to_stage(
                RenderStage::Extract,
                extract_independent_sprites
                    .after(SpriteSystem::ExtractSprites),
            );
        }

    }
}