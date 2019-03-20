use super::system_prelude::*;

pub struct ScaleSpritesSystem;

impl<'s> System<'s> for ScaleSpritesSystem {
    type SystemData = (
        Entities<'s>,
        Read<'s, AssetStorage<SpriteSheet>>,
        ReadStorage<'s, Size>,
        ReadStorage<'s, SpriteRender>,
        WriteStorage<'s, Scale>,
        WriteStorage<'s, Transform>,
    );

    fn run(
        &mut self,
        (mut entities, spritesheet, sizes, sprites, mut scales, mut transforms): Self::SystemData,
    ) {
        let mut to_remove = Vec::new();
        for (entity, size, scale_component, transform, sprite_render) in
            (&*entities, &sizes, &scales, &mut transforms, &sprites).join()
        {
            let spritesheet_handle = &sprite_render.sprite_sheet;
            let sprite_id = sprite_render.sprite_number;
            if let Some(spritesheet) = spritesheet.get(spritesheet_handle) {
                let sprite =
                    spritesheet.sprites.get(sprite_id).expect(&format!(
                        "Couldn't get sprite #{} from spritesheet #{}",
                        sprite_id,
                        spritesheet_handle.id()
                    ));
                let sprite_w = sprite.width;
                let sprite_h = sprite.height;
                let scale = [size.w / sprite_w, size.h / sprite_h];
                transform.set_scale(scale[0], scale[1], 0.0);
                to_remove.push(entity);
            }
        }
        // Remove scale component from scaled entities
        for entity in to_remove {
            scales.remove(entity);
        }
    }
}
