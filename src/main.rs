use std::io;

// Use `WeatherApi` to use `reqwest` Client and use all of the metadata required to query the openweathermap.org api

#[cfg(feature = "cli")]
#[derive(Default, Clone)]

pub struct WeatherApi {
    client: Client,
    api_key: ApiStringType,
    api_endpoint: StringType,
    api_path: StringType
}

fn main() {
    println!("Hello from rusty-weather");

    println!("Please input your local city");

    let mut city = String::new(); // mutable city variable

    io::stdin()
        .read_line(&mut city)
        .expect("Failed to read line");

    println!("The city you entered is: {city}");
}
