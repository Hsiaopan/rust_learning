#[derive(Debug, PartialEq)]
struct Car {
    color: String,
    transmission: Transmission,
    convertible: bool,
    mileage: u32,
}
#[derive(Debug, PartialEq)]
enum Transmission {
    Mannual,
    SemiAuto,
    Automatic,
}

fn car_factory(color: String, transmission: Transmission, convertible: bool) -> Car {
    let car: Car = Car {
        color: color,
        transmission: match transmission {
            Transmission::Mannual => Transmission::Mannual,
            Transmission::SemiAuto => Transmission::SemiAuto,
            Transmission::Automatic => Transmission::Automatic,
        },
        convertible: convertible,
        mileage: 0,
    };

    assert_eq!(car.mileage, 0);
    return car;
}

fn main() {
    let client_request_1 = car_factory(String::from("Red"), Transmission::Mannual, false);
    assert_eq!(client_request_1.color, "Red");
    assert_eq!(client_request_1.transmission, Transmission::Mannual);
    assert_eq!(client_request_1.convertible, false);

    let client_request_2 = car_factory(String::from("Silver"), Transmission::Automatic, true);
    assert_eq!(client_request_2.color, "Silver");
    assert_eq!(client_request_2.transmission, Transmission::Automatic);
    assert_eq!(client_request_2.convertible, true);

    let client_request_3 = car_factory(String::from("Yellow"), Transmission::SemiAuto, true);
    assert_eq!(client_request_3.color, "Yellow");
    assert_eq!(client_request_3.transmission, Transmission::SemiAuto);
    assert_eq!(client_request_3.convertible, true);
}