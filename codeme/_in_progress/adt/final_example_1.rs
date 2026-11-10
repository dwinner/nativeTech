#[derive(Debug)]
struct Point(i32, i32); // Tuple struct for coordinates
#[derive(Debug)]

// Chapter 5
enum VehicleKind {
    Car { passengers: u8 },
    Truck { capacity_tons: f32 },
    Bicycle { gears: u8 },
}

#[derive(Debug)]
struct Vehicle {
    id: u32,
    kind: VehicleKind, // Enum field
    location: Point, // Tuple struct field
}

fn main() {
    let delivery_truck = Vehicle {
        id: 101,
        kind: VehicleKind::Truck { capacity_tons: 2.5 },
        location: Point(50, -30),
    };

    let commuter_bike = Vehicle {
        id: 205,
        kind: VehicleKind::Bicycle { gears: 18 },
        location: Point(10, 15),
    };

    println!("Vehicle 1: {:?}", delivery_truck);
    println!("Vehicle 2: {:?}", commuter_bike);

    // we can match on the kind field
    match &delivery_truck.kind {
        VehicleKind::Truck { capacity_tons } => {
            println!("Truck {} has capacity: {} tons", delivery_truck.id,
                     capacity_tons);
        }
        _ => { // Handle other kinds if necessary
            println!("Vehicle {} is not a truck.", delivery_truck.id);
        }
    }
}