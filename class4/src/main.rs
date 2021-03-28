enum TrafficLight {
    Red,
    Green,
    Yellow
}

impl TrafficLight {
    fn time(&self) -> u8 {
        match self {
            TrafficLight::Yellow => 15,
            TrafficLight::Red => 40,
            TrafficLight::Green => 60,
        }
    }
}

fn sum (arr: &[u32]) -> Option<u32> {
    let mut ret = 0;
    for val in arr {
        ret += val;
    }
    Some(ret)
}


fn main() {
    println!("Hello, traffic light!");
    let red = TrafficLight::Red;
    let green = TrafficLight::Green;
    let yellow = TrafficLight::Yellow;
    println!("{:?}",red.time());
    println!("{:?}",green.time());
    println!("{:?}",yellow.time());

    let arr = [42949672952,22,33,44,55];
    let sum_arr = sum(&arr);
    match sum_arr {
        Some(i) => println!("{:?}", i),
        None => println!("null"),
    }
    println!("{:?}", sum_arr.unwrap());
}

