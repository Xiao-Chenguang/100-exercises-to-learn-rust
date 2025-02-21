// TODO: Flesh out the `WeekTemperatures` struct and its method implementations to pass the tests.

pub struct WeekTemperatures {
    date: [Weekday; 7],
    temp: [Option<i32>; 7],
}

pub enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl WeekTemperatures {
    pub fn new() -> Self {
        WeekTemperatures {
            date: [
                Weekday::Monday,
                Weekday::Tuesday,
                Weekday::Wednesday,
                Weekday::Thursday,
                Weekday::Friday,
                Weekday::Saturday,
                Weekday::Sunday,
            ],
            temp: [None; 7],
        }
    }

    pub fn get_temperature(&self, day: Weekday) -> Option<i32> {
        match day {
            Weekday::Monday => self.temp[0],
            Weekday::Tuesday => self.temp[1],
            Weekday::Wednesday => self.temp[2],
            Weekday::Thursday => self.temp[3],
            Weekday::Friday => self.temp[4],
            Weekday::Saturday => self.temp[5],
            Weekday::Sunday => self.temp[6],
        }
    }

    pub fn set_temperature(&mut self, day: Weekday, temperature: i32) {
        match day {
            Weekday::Monday => self.temp[0] = Some(temperature),
            Weekday::Tuesday => self.temp[1] = Some(temperature),
            Weekday::Wednesday => self.temp[2] = Some(temperature),
            Weekday::Thursday => self.temp[3] = Some(temperature),
            Weekday::Friday => self.temp[4] = Some(temperature),
            Weekday::Saturday => self.temp[5] = Some(temperature),
            Weekday::Sunday => self.temp[6] = Some(temperature),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_temperature() {
        let mut week_temperatures = WeekTemperatures::new();

        assert_eq!(week_temperatures.get_temperature(Weekday::Monday), None);
        assert_eq!(week_temperatures.get_temperature(Weekday::Tuesday), None);
        assert_eq!(week_temperatures.get_temperature(Weekday::Wednesday), None);
        assert_eq!(week_temperatures.get_temperature(Weekday::Thursday), None);
        assert_eq!(week_temperatures.get_temperature(Weekday::Friday), None);
        assert_eq!(week_temperatures.get_temperature(Weekday::Saturday), None);
        assert_eq!(week_temperatures.get_temperature(Weekday::Sunday), None);

        week_temperatures.set_temperature(Weekday::Monday, 20);
        assert_eq!(week_temperatures.get_temperature(Weekday::Monday), Some(20));

        week_temperatures.set_temperature(Weekday::Monday, 25);
        assert_eq!(week_temperatures.get_temperature(Weekday::Monday), Some(25));

        week_temperatures.set_temperature(Weekday::Tuesday, 30);
        week_temperatures.set_temperature(Weekday::Wednesday, 35);
        week_temperatures.set_temperature(Weekday::Thursday, 40);
        week_temperatures.set_temperature(Weekday::Friday, 45);
        week_temperatures.set_temperature(Weekday::Saturday, 50);
        week_temperatures.set_temperature(Weekday::Sunday, 55);

        assert_eq!(week_temperatures.get_temperature(Weekday::Monday), Some(25));
        assert_eq!(
            week_temperatures.get_temperature(Weekday::Tuesday),
            Some(30)
        );
        assert_eq!(
            week_temperatures.get_temperature(Weekday::Wednesday),
            Some(35)
        );
        assert_eq!(
            week_temperatures.get_temperature(Weekday::Thursday),
            Some(40)
        );
        assert_eq!(week_temperatures.get_temperature(Weekday::Friday), Some(45));
        assert_eq!(
            week_temperatures.get_temperature(Weekday::Saturday),
            Some(50)
        );
        assert_eq!(week_temperatures.get_temperature(Weekday::Sunday), Some(55));
    }
}
