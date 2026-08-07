use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::window::PrimaryWindow;

const TILE_SIZE: u32 = 64;//this helps breakdown the sprit sheet into fragments of each animation part.
const TREE_FRAMES: usize = 4;// the number of rows that the tree sprite has.
const SPF : f32 = 0.25;//this tells the seconds per frame
                         
#[derive(Component)]
pub struct Tree;

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Resource)]
pub struct GT(Timer);

#[derive(Component)]
pub struct Escape;

#[derive(Component)]
pub struct TreeScale(pub f32);

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgba(0.0,1.0,0.45,1.0))) 
        .add_plugins(
            DefaultPlugins.set(AssetPlugin {
                file_path: "src/assets".into(),
                ..default()
            }),
        )
        .add_systems(Startup, setup_camera)
        .add_systems(Startup,add_tree)
        .add_systems(Update,(moveoncom,move_tree,bigtree))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn add_tree(mut commands: Commands,
    assests_server: Res<AssetServer>,
    window: Query<&Window, With<PrimaryWindow>>,
    mut atlas_layout: ResMut<Assets<TextureAtlasLayout>>){
    
    let (window_width, window_height) = if let Ok(window) = window.single() {
        (window.width(), window.height())
    } else {
        (1280.0, 720.0)
    };

    let topleftx = -window_width / 2.0;
    let toplefty = window_height / 2.0 ;

    let texture = assests_server.load("Tree1.png");
    let layout = atlas_layout.add(TextureAtlasLayout::from_grid(
        UVec2::splat(TILE_SIZE),
        TREE_FRAMES as u32, // columns used for walking frames
        12,                  // at least 12 rows available
        None,
        None,
    ));

    commands.spawn((
        Text2d::new("Escape..."),
        TextFont {
            font_size: 20.0,
            ..default()
        },
        TextColor(Color::Srgba(bevy::color::Srgba::new(1.0, 1.0, 1.0, 0.0))),
        Anchor::CENTER,
        Transform::from_xyz(topleftx+30.0,toplefty-20.0, 0.0),
        Escape,
        AnimationTimer(Timer::from_seconds(1.5, TimerMode::Repeating)),
    ));

    commands.spawn((
        Sprite::from_atlas_image(
            texture, // the image is given here.
            TextureAtlas {
                layout, // value is added here.
                index: 0, // the index of value where it starts
            },
        ),
        Transform::from_translation(Vec3::ZERO),
        Tree,
        TreeScale(1.0),
        AnimationTimer(Timer::from_seconds(SPF, TimerMode::Repeating)),
    ));
}

fn move_tree(time: Res<Time>,mut query:Query<(&mut AnimationTimer, &mut Sprite),With<Tree>>){
    
    for (mut timer, mut sprite) in query.iter_mut(){
        timer.tick(time.delta());
        //if 15 frames happen, we move a tick up.
        if timer.just_finished(){
            //in sprite inside texture_atlas we have index if let helps us safely get the value from
            //options.
            if let Some(atlas) = &mut sprite.texture_atlas{
                if atlas.index == 3{
                    atlas.index =0;}
                else{
                    atlas.index +=1;}
            }
        }
    }

}

fn moveoncom(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut exit: MessageWriter<AppExit>,
    mut timer : Query<&mut AnimationTimer, With<Escape>>){
    if let Ok(mut gt) = timer.single_mut(){
    if keys.pressed(KeyCode::Escape) {
        gt.0.tick(time.delta());

    }
    else {
        let gtnewval = ( gt.0.elapsed_secs()- time.delta_secs()).max(0.0);
        gt.0.set_elapsed(std::time::Duration::from_secs_f32(gtnewval));
    }
    }
}

fn bigtree(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut TreeScale), With<Tree>>,
) {
    for (mut transform, mut tree_scale) in query.iter_mut() {
        if keys.pressed(KeyCode::ArrowUp) {
            tree_scale.0 += 0.05; 
        }
        if keys.pressed(KeyCode::ArrowDown) {
            tree_scale.0 = (tree_scale.0 - 0.05).max(0.1);
        }

        transform.scale = Vec3::splat(tree_scale.0);
    }
}
