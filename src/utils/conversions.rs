use chrono::{NaiveDate, NaiveDateTime};

/// Converts an ISO8601 datetime string to a human-readable time format.
///
/// Takes a datetime string in the format "YYYY-MM-DDTHH:MM" and converts it
/// to either 12-hour format with AM/PM indicator (e.g., "08:30am") or 24-hour
/// format (e.g., "20:30") based on the hour24 parameter.
///
/// # Arguments
///
/// * `iso8601` - A datetime string in ISO8601 format
/// * `hour24` - If true, returns 24-hour format; if false, returns 12-hour format
///
/// # Returns
///
/// Returns a formatted time string in the requested format.
///
/// # Panics
///
/// Panics if the input string cannot be parsed as a valid ISO8601 datetime.
pub fn iso8601_to_time(iso8601: String, hour24: bool) -> String {
    let dt = NaiveDateTime::parse_from_str(&iso8601, "%Y-%m-%dT%H:%M").unwrap();
    if hour24 {
        dt.format("%H:%M").to_string()
    } else {
        dt.format("%I:%M%P").to_string()
    }
}

/// Converts an ISO8601 date string to a human-readable date format.
///
/// Takes a date string in the format "YYYY-MM-DD" and converts it to
/// a readable format showing the day of week and month/day (e.g., "Mon 03/15").
///
/// # Arguments
///
/// * `iso8601` - A date string in ISO8601 format
///
/// # Returns
///
/// Returns a formatted date string with abbreviated day name and MM/DD format.
///
/// # Panics
///
/// Panics if the input string cannot be parsed as a valid ISO8601 date.
pub fn iso8601_to_date(iso8601: String) -> String {
    let dt = NaiveDate::parse_from_str(&iso8601, "%Y-%m-%d").unwrap();
    dt.format("%a %m/%d").to_string()
}

/// Converts temperature from Celsius to Fahrenheit.
///
/// # Arguments
///
/// * `celsius` - Temperature in Celsius
///
/// # Returns
///
/// Returns temperature in Fahrenheit, rounded to 1 decimal place.
pub fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    (((celsius * 9.0 / 5.0) + 32.0) * 10.0).round() / 10.0
}

/// Converts wind speed from km/h to mph.
///
/// # Arguments
///
/// * `kmh` - Wind speed in kilometers per hour
///
/// # Returns
///
/// Returns wind speed in miles per hour, rounded to 1 decimal place.
pub fn kmh_to_mph(kmh: f64) -> f64 {
    (kmh * 0.621371 * 10.0).round() / 10.0
}

/// Converts precipitation from millimeters to inches.
///
/// # Arguments
///
/// * `mm` - Precipitation in millimeters
///
/// # Returns
///
/// Returns precipitation in inches, rounded to 1 decimal place.
pub fn mm_to_inches(mm: f64) -> f64 {
    (mm * 0.0393701 * 10.0).round() / 10.0
}
