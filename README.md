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

# Create an iCalendar
Create your own iCalendar instance
```rust
let mut ical =  Calendar::create(
    "-//My Business Inc//My Calendar 70.9054//EN",
    "2.0",
    "GREGORIAN",
    "PUBLISH",
    "example@gmail.com",
    "America/New_York");
``` 

# Add events
Add events to the calendar.

```rust
 let mut start_cal:  DateTime<Utc> = Utc::now();
  let date_tz: DateTime<Utc> = Utc::now();
 let start = date_tz.checked_add_signed(Duration::days(2));
   
 match start {
       Some(x) => {
              start_cal = x;
       },
       None => ()
 }
 let own_event = Events{ 
                    
                    dtsart:         start_cal,
                    dtend:          start_cal,
                    dtstamp:        date_tz,
                    uid:            "786566jhjh5546@google.com".to_string(),
                    created:        date_tz,
                    description:    "The description".to_string(),
                    last_modified:  date_tz,
                    location:       "Homestead FL".to_string(),
                    sequence:       0,
                    status:         "CONFIRMED".to_string(),
                    summary:        "My business (Not available)".to_string(),
                    transp:         "OPAQUE".to_string()
                    
    };
 let mut ical =  Calendar::create(
                       "-//My Business Inc//My Calendar 70.9054//EN",
                       "2.0",
                       "GREGORIAN",
                       "PUBLISH",
                       "example@gmail.com",
                       "America/New_York");
 
 ical.add_event(own_event);
 println!("{}", ical.events[0].summary);
 
```

 # iCalendar to a file
Export iCalendar to a file.

```rust
let mut file = std::fs::create("file.ics").expect("Could not create file");
ical.export_to(&mut file).expect("Could not write to stdout");
```

# iCalendar to stdout

```rust
ical.export_to(&mut std::io::stdout()).expect("Could not write to stdout");
```
