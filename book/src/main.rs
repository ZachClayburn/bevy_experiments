use bevy::prelude::*;

mod hello_plugin {
    use bevy::prelude::*;

    #[derive(Component)]
    struct Name(String);

    #[derive(Component)]
    struct Person;

    struct GreetTimer(Timer);

    pub struct HelloPlugin;

    impl Plugin for HelloPlugin {
        fn build(&self, app: &mut App) {
            app.add_startup_system(add_people)
                .insert_resource(GreetTimer(Timer::from_seconds(2.0, true)))
                .add_system(hello_word)
                .add_system(greet_people);
        }
    }

    fn greet_people(
        time: Res<Time>,
        mut timer: ResMut<GreetTimer>,
        query: Query<&Name, With<Person>>,
    ) {
        if timer.0.tick(time.delta()).just_finished() {
            for name in query.iter() {
                println!("Hello {}!", name.0);
            }
        }
    }

    fn add_people(mut commands: Commands) {
        commands
            .spawn()
            .insert(Person)
            .insert(Name("Kaladin Stormblessed".to_owned()));
        commands
            .spawn()
            .insert(Person)
            .insert(Name("Shallan Davar".to_owned()));
        commands
            .spawn()
            .insert(Person)
            .insert(Name("Dalinar Kholin".to_owned()));
    }

    fn hello_word() {
        println!("Hello world!");
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(hello_plugin::HelloPlugin)
        .run();
}
