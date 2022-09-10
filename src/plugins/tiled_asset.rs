use anyhow::Context;
use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
    utils::BoxedFuture,
};
use regex::Regex;
use std::{path::PathBuf, time::Instant};
use tiled::Loader;

pub struct TiledAssetPlugin;
impl Plugin for TiledAssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<TiledAsset>()
            .init_asset_loader::<TiledAssetLoader>();
    }
}

#[derive(Debug, TypeUuid)]
#[uuid = "39cadc56-aa9c-4543-8640-a018b74b5052"]
pub struct TiledAsset {
    pub map: tiled::Map,
    pub x: i32,
    pub y: i32,
}

#[derive(Default)]
struct TiledAssetLoader;
impl AssetLoader for TiledAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            // info!("loading tiled file");
            let start = Instant::now();

            let mut loader = Loader::new();
            let path = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap())
                .join("assets")
                .join(load_context.path());
            let map = loader.load_tmx_map_from(bytes, path)?;

            let re = Regex::new(r"map_x0*(-?\d+)_y0*(-?\d+)\.tmx")?;
            let file_name = load_context
                .path()
                .file_name()
                .context("Cannot get file name")?;
            let file_name = file_name.to_str().context("Cannot converse to str")?;
            let cap = re.captures(file_name).context("Captures file name fail")?;
            let x = cap.get(1).context("fail to get x in captures")?.as_str();
            let y = cap.get(2).context("fail to get y in captures")?.as_str();
            let x: i32 = x.parse()?;
            let y: i32 = y.parse()?;

            load_context.set_default_asset(LoadedAsset::new(TiledAsset { map, x, y }));

            let duration = start.elapsed();
            info!("loaded tiled file: {:?}", duration);
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["tmx"]
    }
}
