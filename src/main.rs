use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use std::cmp;
use bevy::window::PrimaryWindow;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct End;

#[derive(Component)]
struct Name(String);

#[derive(Resource)]
struct GT(Timer);

#[derive(Resource, Default)]
struct TickCounter(u32);

#[derive(Resource, Default)]
struct time_click(Option<f32>);

#[derive(Resource, Default)]
struct sizeupdown(Option<char>);

#[derive(Component)]
pub struct TypewriterText {
    pub full_text: String,
    pub curindex: usize,
    pub timer: Timer,
}

#[derive(Component)]
pub struct speed(f32, f32);

fn main() {
    //app is our container/ the game object. it is what the game is inside of.
    //ECS-entity component system[for each object you give its properties seprately from a curated set

    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(GT(Timer::from_seconds(1.5, TimerMode::Repeating)))
        .init_resource::<time_click>()
        .init_resource::<sizeupdown>()
        .add_systems(Startup, add_people)
        .add_systems(Update, moveoncom)
        .run();
}

fn add_people(mut commands: Commands, window: Query<&Window, With<PrimaryWindow>>) {
    let (window_width, window_height) = if let Ok(window) = window.single() {
        (window.width(), window.height())
    } else {
        (1280.0, 720.0)
    };

    let topleftx = -window_width / 2.0;
    let toplefty = window_height / 2.0 ;
    commands.spawn(Camera2d);
    commands.spawn((
        Text2d::new("Miri"),
        TextFont {
            font_size: 30.0,
            ..default()
        },
        TextColor(Color::Srgba(bevy::color::Srgba::new(1.0, 1.0, 1.0, 1.0))),
        Anchor::CENTER,
        Transform::from_xyz(0.0, 0.0, 0.0),
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
        End
    ));

    commands.spawn((Person, Name("Miri".to_string())));
    commands.spawn((Person, Name("Siya".to_string())));
    commands.spawn((Person, Name("Shraddha".to_string())));
}

//so it is taking all the query names. that also have person as a component

//here for the first tie we are sending a components to another function. will be very usefull in
//the future
fn advance_text_color(color: &mut Color) {
    match color {
        c if *c == Color::srgba(1.0, 1.0, 0.0, 1.0) => *c = Color::srgba(0.0, 1.0, 1.0, 1.0),
        c if *c == Color::srgba(0.0, 1.0, 1.0, 1.0) => *c = Color::srgba(1.0, 0.0, 1.0, 1.0),
        c if *c == Color::srgba(1.0, 0.0, 1.0, 1.0) => *c = Color::srgba(1.0, 1.0, 0.0, 1.0),
        c if *c == Color::srgba(1.0, 1.0, 1.0, 1.0) => *c = Color::srgba(1.0, 1.0, 0.0, 1.0),
        _ => {}
    }
}

//simply this takes the values and move it with command and even escape game same as undertale
fn moveoncom(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut gt : ResMut<GT>,
    mut exit: MessageWriter<AppExit>,
    mut query: Query<(&mut Transform, &mut TextColor), (With<Text2d>, Without<End>)>,
    mut end: Query<&mut TextColor,With<End>>,
) {
    if keys.pressed(KeyCode::Escape) {
        gt.0.tick(time.delta());
        
    }
    else {
        let gtnewval = ( gt.0.elapsed_secs()- time.delta_secs()).max(0.0);
        gt.0.set_elapsed(std::time::Duration::from_secs_f32(gtnewval));
    }

    for mut i in end{
        *i = TextColor(Color::srgba(1.0,1.0,1.0,gt.0.elapsed_secs().min(1.0)));
        if gt.0.just_finished(){
            exit.write(AppExit::Success);
        }
    }

    for (mut place, mut calar) in query.iter_mut() {
        if keys.pressed(KeyCode::KeyW) {
            place.translation.y += 10.0;
        }
        if keys.pressed(KeyCode::KeyD) {
            place.translation.x += 10.0;
        }
        if keys.pressed(KeyCode::KeyS) {
            place.translation.y -= 10.0;
        }
        if keys.pressed(KeyCode::KeyA) {
            place.translation.x -= 10.0;
        }
        if keys.just_pressed(KeyCode::Space) {
            advance_text_color(&mut calar.0);
        }
    }
}

fn change(mut query: Query<&mut Name, With<Person>>) {
    for mut i in query {
        if "Miri" == i.0 {
            i.0 = "little Miri".to_string();
        }
    }
}
