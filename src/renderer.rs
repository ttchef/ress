use crate::piece::Piece;

use macroquad::prelude::*;

pub struct Renderer {
    pub piece_textures: Vec<Texture2D>,
}

impl Renderer {
    pub async fn new() -> Self {
        let mut piece_textures = Vec::new();

        for i in 0..12 {
            let path = Piece::from_index(i).asset_path();
            piece_textures.push(
                load_texture(path.as_str())
                    .await
                    .expect("Piece texture paths are valid"),
            );
        }

        Self { piece_textures }
    }
}
