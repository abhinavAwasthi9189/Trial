use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::window::PrimaryWindow;

#[derive(Component)]
struct Person;

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

fn main() {
    //app is our container/ the game object. it is what the game is inside of.
    //ECS-entity component system[for each object you give its properties seprately from a curated set

    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(GT(Timer::from_seconds(1.0, TimerMode::Repeating)))
        .init_resource::<time_click>()
        .init_resource::<sizeupdown>()
        .add_systems(Startup, (add_people, spawn_dialogue_condition))
        .add_systems(
            Update,
            ((colourchange, sizechange).chain(), hello, update_typewriter),
        )
        .run();
}

pub struct NeoPlug;

impl Plugin for NeoPlug {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_people);
        app.add_systems(Update, /*(change, (hello, change).chain())*/ persec);
    }
}

fn first() {
    println!("hakuna matata");
}

fn add_people(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn((
        Text2d::new("Abhinav"),
        TextFont {
            font_size: 12.0,
            font: default(),
            ..default()
        },
        TextColor(Color::Srgba(bevy::color::Srgba::new(1.0, 1.0, 0.0, 1.0))),
        Transform::from_translation(Vec3::ZERO),
        Person,
    ));
    commands.spawn((Person, Name("Miri".to_string())));
    commands.spawn((Person, Name("Siya".to_string())));
    commands.spawn((Person, Name("Shraddha".to_string())));
}

//so it is taking all the query names. that also have person as a component
fn hello(time: Res<Time>, mut gt: ResMut<GT>, query: Query<&Name, With<Person>>) {
    if gt.0.just_finished() {
        for i in query {
            println!("{}", i.0)
        }
    }
}

fn persec(time: Res<Time>, mut gt: ResMut<GT>, mut click: ResMut<time_click>) {
    if gt.0.just_finished() {
        click.0 = Some(time.elapsed_secs());
        println!("{}", (click.0).unwrap());
    }
}

//it changes the colour of the text2d every second from red->blue->yellow and loop
fn colourchange(
    time: Res<Time>,
    mut gt: ResMut<GT>,
    mut query: Query<&mut TextColor, With<Text2d>>,
) {
    gt.0.tick(time.delta());
    if gt.0.just_finished() {
        for mut i in query {
            //again here query is just referencing
            match *i {
                TextColor(c) if c == Color::srgba(1.0, 1.0, 0.0, 1.0) => {
                    i.0 = Color::srgba(0.0, 1.0, 1.0, 1.0);
                }
                TextColor(c) if c == Color::srgba(0.0, 1.0, 1.0, 1.0) => {
                    i.0 = Color::srgba(1.0, 0.0, 1.0, 1.0);
                }
                TextColor(c) if c == Color::srgba(1.0, 0.0, 1.0, 1.0) => {
                    i.0 = Color::srgba(1.0, 1.0, 0.0, 1.0);
                }

                _ => {}
            }
        }
    }
}

fn sizechange(
    time: Res<Time>,
    mut sud: ResMut<sizeupdown>,
    mut query: Query<(&mut TextFont, &Text2d)>,
) {
    //just know that query is simply as WELL SUD is just smart pointer. we need to choose which
    //value to be shown. that being sud.0
    match sud.0 {
        None => {
            sud.0 = Some('+');

            for (mut text_font, text) in query.iter_mut() {
                if text.0 != "Abhinav" {
                    continue;
                }
                text_font.font_size += time.delta_secs();
            }
        }
        Some(a) => {
            if a == '+' {
                for (mut text_font, text) in query.iter_mut() {
                    if text.0 != "Abhinav" {
                        continue;
                    }

                    text_font.font_size += (time.delta_secs()) * 100.0;
                    if text_font.font_size > 310.0 {
                        sud.0 = Some('-');
                    }
                }
            } else {
                for (mut text_font, text) in query.iter_mut() {
                    if text.0 != "Abhinav" {
                        continue;
                    }
                    text_font.font_size -= (time.delta_secs()) * 100.0;
                    if text_font.font_size < 12.0 {
                        sud.0 = Some('+');
                    }
                }
            }
        }
    }
}

fn spawn_dialogue_condition(mut commands: Commands, windowq: Query<&Window, With<PrimaryWindow>>) {
    let messageme = true; // we will add some kind of condition later. right now. i have no
    // idea.

    if messageme {
        //get the window the primary one.
        if let Ok(window) = windowq.single() {
            // unlike pygame bevy measures distance taking the centre of window as the 0,0
            let x = -(window.width() / 2.0) + 20.0;
            let y = -(window.height() / 2.0) + 30.0;
            commands.spawn((
                Text2d::new(""),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                //this makes so that the part of our object(here text) that is pinned at that
                //coordinate transform is the BottomLeft one.
                Anchor::BOTTOM_LEFT,
                // place the object in terms of xyz.
                Transform::from_xyz(x, y, 1.0),
                TypewriterText {
                    full_text: "Hello Player, how are you!?!".to_string(),
                    curindex: 0, //this tells how many of the index be placed inside of Text2d.
                    timer: Timer::from_seconds(0.4, TimerMode::Repeating),
                },
            ));
        }
    }
}

fn update_typewriter(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut TypewriterText, &mut Text2d)>,
) {
    for (entity, mut typewriter, mut text) in query.iter_mut() {
        typewriter.timer.tick(time.delta());
        //every 0.3 seconds.
        if typewriter.timer.just_finished() {
            if typewriter.curindex < typewriter.full_text.len() {
                typewriter.curindex += 1;

                // Take a substring from index 0 up to our current position, collect and put them
                // inside of the textify
                let textify: String = typewriter
                    .full_text
                    .chars()
                    .take(typewriter.curindex)
                    .collect();

                text.0 = textify;
            } else {
                // Once the full sentence is written out, despawn the entity completely
                commands.entity(entity).despawn();
            }
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
