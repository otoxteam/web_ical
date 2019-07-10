extern crate web_ical;

use web_ical::Calendar;

fn main() {
    let icals = Calendar::new("http://127.0.0.1/basic.ics");
    //http://ical.mac.com/ical/US32Holidays.ics
    for ical in &icals.events{
         println!("Event: {}", ical.summary);
         println!("Started: {}", ical.dtsart.format("%a, %e %b %Y - %T"));
    }
}