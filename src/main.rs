#[derive(PartialEq, PartialOrd)]
struct Satellite {
    name: String,
    velocity: f64,
}

struct SpaceStation {
    name: String,
    crew_size: u8,
    altitude: u32,
}

trait Description {
    //fn describe(&self) -> String; You can do this, or you can create a default trait that your
    //implementations can override, or use the default trait:
    fn describe(&self) -> String {
        return String::from("An object flying through space!");
    }
}

impl Description for Satellite {}

impl Description for SpaceStation {
    fn describe(&self) -> String {
        return format!(
            "the {} flying {} miles high with {} crew members on board!",
            self.name, self.altitude, self.crew_size
        );
    }
}

fn main() {
    let hubble = Satellite {
        name: String::from("Hubble Telescope"),
        velocity: 4.72,
    };
    let iss = SpaceStation {
        name: String::from("International Space Station"),
        crew_size: 6,
        altitude: 254,
    };

    println!("hubble is {}", hubble.describe());
    println!("iss is {}", iss.describe());

    let gps = Satellite {
        name: String::from("GPS"),
        velocity: 2.42,
    };
    println!("hubble == gps is {}", hubble == gps);
    println!("hubble < gps is {}", hubble.velocity < gps.velocity);
}
