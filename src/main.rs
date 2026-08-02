use bevy::prelude::*;
use std::sync::atomic::{AtomicU32, Ordering};

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

#[derive(Resource)]
struct GT(Timer);

//these are resources, unlike componenets a certain resource can only be one in code. the default
//makes so that we have a default/ remember the default value.
#[derive(Resource, Default)]
struct TickCounter(u32);

#[derive(Resource, Default)]
struct time_click(Option<f32>);

fn main() {
    //app is our container/ the game object. it is what the game is inside of.
    //ECS-entity component system[for each object you give its properties seprately from a curated set

    App::new()
        .add_plugins(DefaultPlugins)
        // bith insert/init allow us to insert the resources. unlike insert, init can be initialized
        // without assigning value. so it just puts in the default value.
        // here GT gets a timer type value which as a limit of 1.0 sec so above it and ot goes back
        // to 0.0 repeating means we can keep doing it.
        .insert_resource(GT(Timer::from_seconds(1.0, TimerMode::Repeating)))
        .init_resource::<time_click>()
        .add_plugins(NeoPlug)
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
    commands.spawn((Person, Name("Miri".to_string())));
    commands.spawn((Person, Name("Siya".to_string())));
    commands.spawn((Person, Name("Shraddha".to_string())));
}

//so it is taking all the query names. that also have person as a component
//this is wrong but i was just trying stuff.
fn hello(query: Query<&Name, With<Person>>) {
    static TIME: AtomicU32 = AtomicU32::new(0);
    if TIME.load(Ordering::Relaxed) > 60 {
        for i in query {
            println!("{}", i.0)
        }
        TIME.fetch_sub(60, Ordering::Relaxed);
    }
    TIME.fetch_sub(1, Ordering::Relaxed);
}

//here Res allows us to get resources. unlike componets which all are in Query. they have to be
//taken seprately. even choose if mutable or immutable.
fn persec(time: Res<Time>, mut gt: ResMut<GT>, mut click: ResMut<time_click>) {
    // this function just adds what more value we have gotten from last time to the gt value
    gt.0.tick(time.delta());
    // and here just finished checks if after that addition did the value went beyond 1 as set
    // before.
    if gt.0.just_finished() {
        // elapsed value allows us to get a time components.
        // we need to do  .0 to access the value if yoy struct name(datatype) it,
        click.0 = Some(time.elapsed_secs());
        println!("{}", (click.0).unwrap() as i32);
    }
}

fn change(mut query: Query<&mut Name, With<Person>>) {
    for mut i in query {
        if "Miri" == i.0 {
            i.0 = "little Miri".to_string();
        }
    }
}
