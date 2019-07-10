# web_ical

`web_ical` is an esay iCalendar Rust library. It’s goals are to read and write ics web files (Google Calendar, Airbnb Calendar and more) data in a developer-friendly way.

# Examples 1
```rust
extern crate web_ical;

use web_ical::Calendar;

fn main() {
    let icals = Calendar::new("http://ical.mac.com/ical/US32Holidays.ics");

    for ical in &icals.events{
         println!("Event: {}", ical.summary);
         println!("Started: {}", ical.dtsart.format("%a, %e %b %Y - %T"));
    }
}
```
# Examples 2
```rust
extern crate web_ical;

use web_ical::Calendar;

fn main() {
    let icals = Calendar::new("http://ical.mac.com/ical/US32Holidays.ics");
     println!("UTC now is: {}", icals.events[0].dtsart); 
     println!("UTC now in RFC 2822 is: {}", icals.events[0].dtsart.to_rfc2822()); 
     println!("UTC now in RFC 3339 is: {}", icals.events[0].dtsart.to_rfc3339()); 
     println!("UTC now in a custom format is: {}", icals.events[0].dtsart.format("%a %b %e %T %Y"));
}
```
