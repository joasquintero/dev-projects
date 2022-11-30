use bevy::prelude::*;

#[derive(Component)]
struct Position { x: f32, y: f32 }
#[derive(Component)]
struct Person;
#[derive(Component)]
struct Name(String);



fn print_position_system(query: Query<&Transform>) {
    for transform in query.iter() {
        println!("position: {:?}", transform.translation);
    }
}

fn hello_world() {
    println!("hello world!");
}


fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in query.iter() {
        println!("hello {}!", name.0);
    }
}


fn main() {
    App::new()
        .add_startup_system(add_people)
        .add_system(hello_world)
        .add_system(greet_people)
        .run();
}

