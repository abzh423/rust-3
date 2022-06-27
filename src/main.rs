use std::error::Error;
use std::io::{self, stderr, BufRead, Write};

fn main() {
    let mut total = 0;
    // compound assignment
    // there are no increment or decrement operators in rust
    total += 10;
    total += 10;
    // conversion to usize
    println!("{}", total as usize);

    // simple example of a closure
    // they are just like callbacks in javascript
    let is_even = |x: usize| -> bool { x % 2 == 0 };
    println!("{}", is_even(2));

    // A program panics when something so messed
    // up happens there must be a bug in the
    // program itself
    // a good rule of thumb is to not panic
    fn pirate_share(total: u64, crew_size: usize) -> u64 {
        let half = total / 2;
        half / crew_size as u64
    }

    // Returns 'Result' type
    // fn get_weather(location: String) -> Result<WeatherReport, io::Error> {}
    // The most through way to deal with 'Result' is match expression
    // match get_weather(hometown) {
    //     Ok(report) => display_weather(hometown, &report),
    //     Err(err) => {
    //         println!("error querying the weather: {}", err);
    //         schedule_weather_retry();
    //     }
    // }
    //
    // result.is_ok();
    // result.is_err();

    fn print_error(mut err: &dyn Error) {
        let _ = writeln!(stderr(), "error: {}", err);
        while let Some(source) = err.source() {
            let _ = writeln!(stderr(), "caused by: {}", source);
            err = source;
        }
    }

    type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;
    type GenericResult<T> = Result<T, GenericError>;

    fn read_numbers(file: &mut dyn BufRead) -> GenericResult<Vec<i64>> {
        let mut numbers = vec![];
        for line_result in file.lines() {
            let line = line_result?;
            numbers.push(line.parse()?);
        }
        Ok(numbers)
    }

    let io_error = io::Error::new(io::ErrorKind::Other, "timed out");

    // loop {
    //     match compile_project() {
    //         Ok(()) => return Ok(),
    //         Err(err) => {
    //             if let Some(mse) = err.downcast_ref::<MissingSemicolonError>() {
    //                 insert_semicolon_in_source_code(mse.file(), mse.line())?;
    //                 continue;
    //             }
    //             return Err(err);
    //         }
    //     }
    // }
}
