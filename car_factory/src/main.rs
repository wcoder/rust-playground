
#[derive(PartialEq, Debug)]
struct Car {
    color: String,
    motor: Transmission,
    roof: bool,
    age: (Age, u32),
}

#[derive(PartialEq, Debug)]
enum Transmission {
    Automatic,
    SemiAuto,
    Manual,
}

#[derive(PartialEq, Debug)]
enum Age {
    New,
    Used,
}

// Build a "Car" by using values from the input arguments
fn car_factory(order: i32, miles: u32) -> Car {

    let colors = ["Blue", "Green", "Red", "Silver"];

    let mut color = order as usize;
    if color > 4 {
        // color = 5 --> index 1, 6 --> 2, 7 --> 3, 8 --> 4
        color = color - 4;
    }

    // Add variety to orders for motor type and roof type
    let mut motor = Transmission::Manual;
    let mut roof = true;
    if order % 3 == 0 {          // 3, 6, 9
        motor = Transmission::Automatic;
    } else if order % 2 == 0 {   // 2, 4, 8, 10
        motor = Transmission::SemiAuto;
        roof = false;
    }                            // 1, 5, 7, 11

    // Return requested "Car"
    Car {
        color: String::from(colors[(color-1) as usize]),
        motor: motor,
        roof: roof,
        age: car_quality(miles)
    }
}

fn car_quality(miles: u32) -> (Age, u32) {
    let age = if miles > 0 {
        Age::Used
    } else {
        Age::New
    };
    return (age, miles);
}

fn main() {
    use std::collections::HashMap;
    let mut orders: HashMap<i32, Car> = HashMap::new();

    // Initialize counter variable
    let mut order = 1;
    // Declare a car as mutable "Car" struct
    let mut car: Car;

    // Order 6 cars
    // - Increment "order" after each request
    // - Add each order <K, V> pair to "orders" hash map
    // - Corrected code: Use ".insert()" method to add each order
    // - Adjust println call to show order details from the hash map

    // Car order #1: Used, Hard top
    car = car_factory(order, 1000);
    orders.insert(order, car);
    println!("Car order {}: {:?}", order, orders.get(&order));

    // Car order #2: Used, Convertible
    order = order + 1;
    car = car_factory(order, 2000);
    orders.insert(order, car);
    println!("Car order {}: {:?}", order, orders.get(&order));

    // Car order #3: New, Hard top
    order = order + 1;
    car = car_factory(order, 0);
    orders.insert(order, car);
    println!("Car order {}: {:?}", order, orders.get(&order));

    // Car order #4: New, Convertible
    order = order + 1;
    car = car_factory(order, 0);
    orders.insert(order, car);
    println!("Car order {}: {:?}", order, orders.get(&order));

    // Car order #5: Used, Hard top
    order = order + 1;
    car = car_factory(order, 3000);
    orders.insert(order, car);
    println!("Car order {}: {:?}", order, orders.get(&order));

    // Car order #6: Used, Hard top
    order = order + 1;
    car = car_factory(order, 4000);
    orders.insert(order, car);
    println!("Car order {}: {:?}", order, orders.get(&order));
}
