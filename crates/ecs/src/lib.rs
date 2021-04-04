mod world;

use world::{
    World,
    Health,
    Name
};

pub fn ecs_example() {
    println!("Hello, world!");

    let mut world = World::new();

    let entity0 = world.new_entity();
    world.add_component_to_entity(entity0, Health(1000));
    world.add_component_to_entity(entity0, Name("Pure Vessel"));

    let entity2 = world.new_entity();
    world.add_component_to_entity(entity2, Name("The Radiance"));

    let entity1 = world.new_entity();
    world.add_component_to_entity(entity1, Health(-10));
    world.add_component_to_entity(entity1, Name("The Knight"));

    let mut healths = world.borrow_component_vec_mut::<Health>().unwrap();
    let mut names = world.borrow_component_vec_mut::<Name>().unwrap();
    let zip = healths.iter_mut().zip(names.iter_mut());
    let iter = zip.filter_map(|(health, name)| Some((health.as_mut()?, name.as_mut()?)));

    for (health, name) in iter {
        if health.0 <= 0 {
            println!("{} has Perished", name.0);
            println!("Reviving {} with 100 health", name.0);
            *health = Health(100);
        } else {
            println!("{} still Standing", name.0)
        }
    }
}
