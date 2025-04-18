#[allow(unused)]
struct Employee {
    name: String,
    atr: u32,
}

#[allow(unused)]
fn show_data() {
    let data = Employee {
        name: "Tom".to_string(),
        atr: 592,
    };
}

// Another example 

const MAXIMUM_POWER :u16 = 1000;
enum VehicleKind{
    Car,
    Bike,
    Truck
}

struct VehicleData {
    kind: VehicleKind,
    registration_year: u16,
    registration_month : u8,
    power:u16
}

fn vehicle(){
let vehicle = VehicleData {
    kind: VehicleKind::Car,
    registration_year: 2003,
    registration_month: 11,
    power: 120,
    };
    
    if vehicle.power > MAXIMUM_POWER {
    println!("Too powerful");
    }
}