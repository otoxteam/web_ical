extern crate chrono;
extern crate web_ical;

use chrono::Utc;
use chrono::{DateTime, Duration};
use web_ical::Calendar;
use web_ical::Events;

fn main() {
    let icals = Calendar::new("http://ical.mac.com/ical/US32Holidays.ics").unwrap();
    //http://ical.mac.com/ical/US32Holidays.ics
    for ical in &icals.events {
        println!("Event: {}", ical.summary);
        println!("Started: {}", ical.dtsart.format("%a, %e %b %Y - %T"));
    }

    let mut start_cal: DateTime<Utc> = Utc::now();
    let date_tz: DateTime<Utc> = Utc::now();
    let start = date_tz.checked_add_signed(Duration::days(2));

    if let Some(x) = start {
        start_cal = x;
    }

    let own_event = Events {
        dtsart: start_cal,
        dtend: start_cal,
        dtstamp: date_tz,
        uid: "786566jhjh5546@google.com".to_string(),
        created: date_tz,
        description: "The description".to_string(),
        last_modified: date_tz,
        location: "Homestead FL".to_string(),
        sequence: 0,
        status: "CONFIRMED".to_string(),
        summary: "My business (Not available)".to_string(),
        transp: "OPAQUE".to_string(),
    };
    let mut ical = Calendar::create(
        "-//My Business Inc//My Calendar 70.9054//EN",
        "2.0",
        "GREGORIAN",
        "PUBLISH",
        "otoxteam@gmail.com",
        "America/New_York",
    );

    ical.add_event(own_event);
    println!("{}", ical.events[0].summary);

    match ical.export_ics("ical.ics") {
        Ok(_) => println!("OK"),
        Err(e) => panic!("Err: {}", e),
    };
}
