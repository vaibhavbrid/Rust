enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    fn duration(&self) -> u32 {
        match self {
            TrafficLight::Red => 60,
            TrafficLight::Yellow => 5,
            TrafficLight::Green => 55,
        }
    }
}

fn main() {
    let light= TrafficLight::Red;
    println!("The duration of light is {} seconds.", light.duration());
}
