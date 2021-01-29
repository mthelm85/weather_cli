mod api_call;
use chrono::offset::Utc;
use chrono::TimeZone;
use chrono::Local;
mod user_input;

fn main() {
    let input = user_input::get_user_input();
    let jsonval = api_call::get_weather(&input.city, &input.state);
    match jsonval {
        Ok(v) => {
            if v["cod"] == 200 {
                println!("Current Conditions: {}", v["weather"][0]["description"]);
                println!("Temperature: {}°", v["main"]["temp"]);
                println!("Feels Like: {}°", v["main"]["feels_like"]);
                println!("Humidity: {}%", v["main"]["humidity"]);
                println!("Wind: {} mph", v["wind"]["speed"]);
                println!("Gust: {} mph", v["wind"]["gust"]);
                if input.daylight {
                    let sunrise = match v["sys"]["sunrise"].as_i64() {
                        Some(num) => Utc.timestamp(num, 0).with_timezone(&Local).time().format("%I:%M:%p"),
                        None => Utc.timestamp(0,0).with_timezone(&Local).time().format("%I:%M:%p")
                    };
                    let sunset = match v["sys"]["sunset"].as_i64() {
                        Some(num) => Utc.timestamp(num, 0).with_timezone(&Local).time().format("%I:%M:%p"),
                        None => Utc.timestamp(0,0).with_timezone(&Local).time().format("%I:%M:%p")
                    };
                    println!("Sunrise: {}", sunrise);
                    println!("Sunset: {}", sunset);
                }
            } else {
                println!("Error: {}", v["message"])
            }
        }
        Err(err) => println!("{:#?}", err)
    }
}