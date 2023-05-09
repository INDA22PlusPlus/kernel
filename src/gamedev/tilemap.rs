use crate::gamedev::sprite::Sprite;
use crate::graph::surface::Surface;

pub struct TileMap {
    bg: Surface,
    // tile_sprites: *mut Sprite,           // Should work will kalloc
    // amount_of_different_tiles: usize,    // to be used with kalloc to have the right size

    // TODO: Test that this solution works
    tile_sprites: [Sprite],
    tile_map: [[usize]],
}