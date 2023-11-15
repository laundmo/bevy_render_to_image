use bevy::{
    prelude::*,
    render::{
        camera::RenderTarget,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
    },
    window::WindowResolution,
    winit::WinitSettings,
};
use bevy_image_export::{ImageExportBundle, ImageExportPlugin, ImageExportSource};
use std::f32::consts::PI;

fn main() {
    App::new()
        .insert_resource(WinitSettings {
            return_from_run: true,
            ..default()
        })
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::new(768.0, 768.0).with_scale_factor_override(1.0),
                    ..default()
                }),
                ..default()
            }),
            ImageExportPlugin,
        ))
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0,
        })
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
    mut export_sources: ResMut<Assets<ImageExportSource>>,
) {
    let output_texture_handle = {
        let size = Extent3d {
            width: 768,
            height: 768,
            ..default()
        };
        let mut export_texture = Image {
            texture_descriptor: TextureDescriptor {
                label: None,
                size,
                dimension: TextureDimension::D2,
                format: TextureFormat::Rgba8UnormSrgb,
                mip_level_count: 1,
                sample_count: 1,
                usage: TextureUsages::COPY_DST
                    | TextureUsages::COPY_SRC
                    | TextureUsages::RENDER_ATTACHMENT,
                view_formats: &[],
            },
            ..default()
        };
        export_texture.resize(size);

        images.add(export_texture)
    };

    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_translation(4.2 * Vec3::Z),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(Camera3dBundle {
                camera: Camera {
                    target: RenderTarget::Image(output_texture_handle.clone()),
                    ..default()
                },
                ..default()
            });
        });

    commands.spawn(ImageExportBundle {
        source: export_sources.add(output_texture_handle.into()),
        ..default()
    });

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::try_from(shape::Cube::default()).unwrap()),
            material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
            ..default()
        },
        Moving,
    ));
}

#[derive(Component)]
struct Moving;
fn update(mut transforms: Query<&mut Transform, With<Moving>>, mut frame: Local<u32>) {
    let theta = *frame as f32 * 0.25 * PI;
    *frame += 1;
    for mut transform in &mut transforms {
        transform.translation = Vec3::new(theta.sin(), theta.cos(), 0.0);
    }
}
