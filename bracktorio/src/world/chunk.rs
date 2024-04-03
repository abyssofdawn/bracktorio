use super::*;

#[derive(Component)]
pub struct Chunk {
    pub x: i32,
    pub y: i32,
    pub visual_tiles: Vec<Tile>,
    pub objects: Vec<usize>,
    pub dirty: bool,
}

pub struct ChunkVisualTiles {}

impl <'a> System<'a> for ChunkVisualTiles {
    type SystemData = (
        WriteStorage<'a, Chunk>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut chunks,)= data;
        for chunk in (&mut chunks).join() {
            if chunk.dirty {
                chunk.dirty = false;
                let mut visual_tiles: Vec<Tile> = vec![Tile{
                    glyph: 2,
                    fg: RGB::named(RED),
                    bg: RGB::named(BLACK),
                }; 64];
                for id in &chunk.objects {
                    
                }
            }
        }
    }
}