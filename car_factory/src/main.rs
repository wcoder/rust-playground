
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
    while color > 4 {
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

    // Declare a car as mutable "Car" struct
    let mut car: Car;

    // Order 11 cars
    // Corrected code: Use "for" loop to fulfill orders for 11 cars
    // Corrected code: "order" variable initialized and incremented in "for" loop

    // Start with zero miles
    let mut miles = 0;

    for order in 1..12 {

        // Call car_factory to fulfill order
        // Add order <K, V> pair to "orders" hash map
        // Call println! to show order details from the hash map
        car = car_factory(order, miles);
        orders.insert(order, car);
        println!("Car order {}: {:?}", order, orders.get(&order));

        // Reset miles for order variety
        if miles == 2100 {
            miles = 0;
        } else {
            miles = miles + 700;
        }
    }
}
