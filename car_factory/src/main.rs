
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
// - color (String)
// - motor (Transmission enum)
// - roof (boolean, true if the car has a hard top roof)
// - miles (u32)
fn car_factory(color: String, motor: Transmission, roof: bool, miles: u32) -> Car {

    if car_quality(miles).0 == Age::Used {
        if roof {
            println!("Preparing a used car: {:?}, {}, Hard top, {} miles", motor, color, miles);
        } else {
            println!("Preparing a used car: {:?}, {}, Convertible, {} miles", motor, color, miles);
        }
    } else {
        if roof {
            println!("Building a new car: {:?}, {}, Hard top, {} miles", motor, color, miles);
        } else {
            println!("Building a new car: {:?}, {}, Convertible, {} miles", motor, color, miles);
        }
    }

    return Car {
        color: color,
        motor: motor,
        roof: roof,
        age: car_quality(miles),
    };
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
    car_factory(String::from("Orange"), Transmission::Manual, true, 0);

    car_factory(String::from("Red"), Transmission::SemiAuto, false, 565);

    car_factory(String::from("White"), Transmission::Automatic, true, 3000);
}
