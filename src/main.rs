mod api_call;
use chrono::TimeZone;
mod user_input;

fn main() {
    let input = user_input::get_user_input();
    let res = api_call::get_weather(&input.city, &input.state);
    match res {
        Ok(r) => {
            match r {
                api_call::Response::Success(r) => {
                    println!("Current Conditions: {}", r.weather[0].main);
                    println!("Temperature: {}°", r.main.temp);
                    println!("Feels Like: {}°", r.main.feels_like);
                    println!("Humidity: {}%", r.main.humidity);
                    println!("Wind: {} mph", r.wind.speed);
                    println!("Gust: {:?} mph", r.wind.gust);
                    if input.daylight {
                        let sunrise = chrono::FixedOffset::east(r.timezone).timestamp(r.sys.sunrise, 0).format("%I:%M:%p");
                        let sunset = chrono::FixedOffset::east(r.timezone).timestamp(r.sys.sunset, 0).format("%I:%M:%p");
                        println!("Sunrise: {}", sunrise);
                        println!("Sunset: {}", sunset);
                    }
                },
                api_call::Response::Error(e) => {
                    println!("Error: {}", e.message);
                }

            }
        },
        Err(err) => println!("{:#?}", err)
    }
}