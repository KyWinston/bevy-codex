use bevy::{
    core_pipeline::fxaa::Fxaa,
    pbr::ScreenSpaceReflectionsBundle,
    prelude::*,
    render::render_resource::{
        Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
    },
    window::PrimaryWindow,
};
use bevy_lunex::{
    prelude::{MainUi, Pickable, Rl, UiNodeTreeInitTrait, UiTree},
    Base, MovableByCamera, PackageLayout, PickingPortal, UiImage2dBundle, UiLayout, UiLink,
    UiTreeBundle,
};

use super::components::{Hud, UiDisplay};
use crate::components::MainCam;

pub fn build_hud(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    query: Query<Entity, Added<Hud>>,
) {
    for route_entity in &query {
        if let Ok(window) = primary_window.get_single() {
            let (env_pth, env_suffix) = ("images/environment_maps/pisa_", "_rgb9e5_zstd.ktx2");
            let resolution = &window.resolution;
            let w_size = (resolution.width(), resolution.height());
            let size = Extent3d {
                width: w_size.0 as u32,
                height: w_size.1 as u32,
                ..default()
            };

            let mut image = Image {
                texture_descriptor: TextureDescriptor {
                    label: None,
                    size,
                    dimension: TextureDimension::D2,
                    format: TextureFormat::Bgra8UnormSrgb,
                    mip_level_count: 1,
                    sample_count: 1,
                    usage: TextureUsages::TEXTURE_BINDING
                        | TextureUsages::COPY_DST
                        | TextureUsages::RENDER_ATTACHMENT,
                    view_formats: &[],
                },
                ..default()
            };

            // Spawn the route
            commands
                .entity(route_entity)
                .insert(SpatialBundle::default())
                .with_children(|route| {
                    image.resize(size);
                    let render_image = asset_server.add(image);
                    // route
                    // .spawn((SpatialBundle::default(), MainCam))
                    // .with_children(|route| {
                    // Spawn 3D camera
                    route.spawn((
                        MainCam,
                        Camera3dBundle {
                            camera: Camera {
                                order: -1,
                                target: render_image.clone().into(),
                                clear_color: ClearColorConfig::Default,
                                ..default()
                            },
                            projection: Projection::Perspective(PerspectiveProjection {
                                fov: 60.0_f32.to_radians(),
                                ..default()
                            }),
                            ..default()
                        },
                        VisibilityBundle::default(),
                        ScreenSpaceReflectionsBundle::default(),
                        Fxaa::default(),
                        EnvironmentMapLight {
                            diffuse_map: asset_server
                                .load(env_pth.to_string() + "diffuse" + env_suffix),
                            specular_map: asset_server
                                .load(env_pth.to_string() + "specular" + env_suffix),
                            intensity: 400.0,
                        },
                    ));

                    // Spawn the background
                    route
                        .spawn((
                            UiTreeBundle::<MainUi>::from(UiTree::new2d("HUD")),
                            UiLayout::window().size(Rl(w_size)).pack::<Base>(),
                            MovableByCamera,
                            UiDisplay,
                        ))
                        .with_children(|ui| {
                            // Spawn 3D camera view
                            ui.spawn((
                                UiLink::<MainUi>::path("Camera"),
                                UiLayout::window().size(Rl(100.0)).pack::<Base>(), // Make this resizable
                                UiImage2dBundle::from(render_image),
                                PickingPortal,
                            ));
                            ui.spawn((
                                UiLink::<MainUi>::path("Camera/Hud"),
                                UiLayout::window_full().pack::<Base>(),
                                Pickable::IGNORE,
                            ));
                        });
                });
        }
    }
}
