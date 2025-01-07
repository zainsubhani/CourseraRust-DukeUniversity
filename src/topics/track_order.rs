struct Car {
    model: String,
    year: u32,
    color: String,
    rating: f32,
    genuine: bool,
    type_of_car: type_of_car,
    age: (Age, u32),
}
enum type_of_car {
    Sedan,
    Hatchback,
    SUV,
    Coupe,
    Convertible,
    Wagon,
    Van,
    Truck,
    Minivan,
    Crossover,
    Electric,
    Hybrid,
    Sports,
    Luxury,
    Compact,
    Off_Road,
    Pickup,
    Micro,
    Limousine,
    Roadster,
    Supercar,
    Muscle,
    Classic,
    Vintage,
    Antique,
    Exotic,
    Kit,
    Tuner,
    Hot_Rod,
    Rat_Rod,
    Lowrider,
    Restomod,
    Stock,
    Custom,
    Restored,
    Survivor,
    Barn_Find,
    Project,
    Parts_Car,
}
enum Age {
    New,
    Used,
    Certified_Pre_Owned,
    Salvage,
    Rebuilt,
    Flood_Damaged,
    Lemon,
    Junk,
    Parts_Only,
    Not_Specified,
}
fn car_quality(miles: u32) -> (Age, u32) {
    if miles < 1000 {
        return (Age::New, miles);
    } else if miles < 10000 {
        return (Age::Used, miles);
    } else if miles < 50000 {
        return (Age::Certified_Pre_Owned, miles);
    } else if miles < 100000 {
        return (Age::Salvage, miles);
    } else if miles < 200000 {
        return (Age::Rebuilt, miles);
    } else if miles < 300000 {
        return (Age::Flood_Damaged, miles);
    } else if miles < 400000 {
        return (Age::Lemon, miles);
    } else if miles < 500000 {
        return (Age::Junk, miles);
    } else if miles < 600000 {
        return (Age::Parts_Only, miles);
    } else {
        return (Age::Not_Specified, miles);
    }
    (Age::New, miles)
}
// Build "Car" using input arguments
fn car_factory(order: i32, miles: u32) -> Car {
    let colors = ["Blue", "Green", "Red", "Silver"];

    // Prevent panic: Check color index for colors array, reset as needed
    // Valid color = 1, 2, 3, or 4
    // If color > 4, reduce color to valid index
    let mut color = order as usize;
    if color > 4 {
        // color = 5 --> index 1, 6 --> 2, 7 --> 3, 8 --> 4
        color = color - 4;
    }

    // Add variety to orders for motor type and roof type
    let mut motor = type_of_car::Sedan;
    let mut genuine = true;
    if order % 3 == 0 {
        // 3, 6, 9
        motor = type_of_car::Sports;
    } else if order % 2 == 0 {
        // 2, 4, 8, 10
        motor = type_of_car::SemiAuto;
        roof = false;
    } // 1, 5, 7, 11

    // Return requested "Car"
    Car {
        model: format!("Model {}", order),
        year: 2021,
        color: colors[color - 1].to_string(),
        rating: 4.5,
        genuine: genuine,
        type_of_car: motor,
        age: car_quality(miles),
    }
}
fn main() {
    // Initialize counter variable
    let mut order = 1;
    // Declare a car as mutable "Car" struct
    let mut car: Car;

    // Order 6 cars, increment "order" for each request
    // Car order #1: Used, Hard top
    car = car_factory(order, 1000);
    println!(
        "{}: {:?}, Hard top = {}, {:?}, {}, {} miles",
        order, car.age.0, car.roof, car.motor, car.color, car.age.1
    );

    // Car order #2: Used, Convertible
    order = order + 1;
    car = car_factory(order, 2000);
    println!(
        "{}: {:?}, Hard top = {}, {:?}, {}, {} miles",
        order, car.age.0, car.roof, car.motor, car.color, car.age.1
    );

    // Car order #3: New, Hard top
    order = order + 1;
    car = car_factory(order, 0);
    println!(
        "{}: {:?}, Hard top = {}, {:?}, {}, {} miles",
        order, car.age.0, car.roof, car.motor, car.color, car.age.1
    );

    // Car order #4: New, Convertible
    order = order + 1;
    car = car_factory(order, 0);
    println!(
        "{}: {:?}, Hard top = {}, {:?}, {}, {} miles",
        order, car.age.0, car.roof, car.motor, car.color, car.age.1
    );

    // Car order #5: Used, Hard top
    order = order + 1;
    car = car_factory(order, 3000);
    println!(
        "{}: {:?}, Hard top = {}, {:?}, {}, {} miles",
        order, car.age.0, car.roof, car.motor, car.color, car.age.1
    );

    // Car order #6: Used, Hard top
    order = order + 1;
    car = car_factory(order, 4000);
    println!(
        "{}: {:?}, Hard top = {}, {:?}, {}, {} miles",
        order, car.age.0, car.roof, car.motor, car.color, car.age.1
    );
}
