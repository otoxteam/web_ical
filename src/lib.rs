//! # web_ical
//!
//! `web_ical` is an esay iCalendar Rust library. It’s goals are to read and write ics web files (Google Calendar, Airbnb Calendar and more) data in a developer-friendly way.
//!
//! # Examples 1
//! ```
//! extern crate web_ical;
//!
//!use web_ical::Calendar;
//!
//!fn main() {
//!    let icals = Calendar::new("http://ical.mac.com/ical/US32Holidays.ics");
//!
//!    for ical in &icals.events{
//!         println!("Event: {}", ical.summary);
//!         println!("Started: {}", ical.dtsart.format("%a, %e %b %Y - %T"));
//!    }
//!}
//! ```
//! # Examples 2
//! ```
//! extern crate web_ical;
//!
//!use web_ical::Calendar;
//!
//!fn main() {
//!    let icals = Calendar::new("http://ical.mac.com/ical/US32Holidays.ics");
//!     println!("UTC now is: {}", icals.events[0].dtsart); 
//!     println!("UTC now in RFC 2822 is: {}", icals.events[0].dtsart.to_rfc2822()); 
//!     println!("UTC now in RFC 3339 is: {}", icals.events[0].dtsart.to_rfc3339()); 
//!     println!("UTC now in a custom format is: {}", icals.events[0].dtsart.format("%a %b %e %T %Y"));
//!}
//! ```
extern crate chrono;
extern crate regex;

use regex::RegexSet;
use chrono::{DateTime, NaiveDateTime};
use chrono::{Utc};

///Convert datetime string to [`DateTime`](https://docs.rs/chrono/0.4.7/chrono/struct.DateTime.html) 
/// 
/// # Examples
/// 
/// ```
/// let result_obj_aux: Result<DateTime<Utc>, String>;
/// result_obj_aux = convert_datetime("20190522T232701Z", "%Y%m%dT%H%M%SZ".to_string());
/// match result_obj_aux{
///     Ok(val) => {
///             println!("{}", val);
///         },
///     Err(_) => (),
///}        
///```
pub fn convert_datetime(value: String, format: String) -> Result<DateTime<Utc>, String> {
    let no_timezone_aux = NaiveDateTime::parse_from_str(&value, &format).unwrap();
    let date_tz_axu: DateTime<Utc> = DateTime::from_utc(no_timezone_aux, Utc);
    Ok(date_tz_axu)
}

///store all events from iCalendar.
#[derive(Clone)]
pub struct Events{
    pub begin:          String,
    pub dtsart:         DateTime<Utc>,
    pub dtend:          DateTime<Utc>,
    pub dtstamp:        DateTime<Utc>,
    pub uid:            String,
    pub created:        DateTime<Utc>,
    pub description:    String,
    pub last_modified:  DateTime<Utc>,
    pub location:       String,
    pub sequence:       u32,
    pub status:         String,
    pub summary:        String,
    pub transp:         String,
    pub end:            String
 }
 impl Events{
///Check if the events is all day.
        pub fn is_all_day(&self) -> bool{
            let dur = self.dtend.signed_duration_since(self.dtsart);
            if dur.num_hours() >= 24 {
                true
            }else{
                false
            }
        }
 }

 ///store the iCalendar and set events to struct `Events`.
 #[derive(Clone)]
 pub struct Calendar{
    pub begin:          String,
    pub prodid:         String,
    pub version:        String,
    pub calscale:       String,
    pub method:         String,
    pub x_wr_calname:   String,
    pub x_wr_timezone:  String,
    pub events:         Vec<Events>,
    pub end:            String
}

impl Calendar{
///Request HTTP or HTTPS to iCalendar url.
    pub fn new(url: &'static str) -> Calendar{
        let response_text = reqwest::get(url)
                            .expect("Could not make request")
                            .text()
                            .expect("Could not read response");
        let  text_data: Vec<_>; 
        if response_text.contains("\r\n"){
            text_data = response_text.split("\r\n").collect::<Vec<_>>();
        }else{
            text_data = response_text.split("\n").collect::<Vec<_>>();
        }
        let no_timezone = NaiveDateTime::parse_from_str("20190630T130000Z", "%Y%m%dT%H%M%SZ").unwrap();
        let date_tz: DateTime<Utc> = DateTime::from_utc(no_timezone, Utc);
        let mut struct_even: Vec<Events> = Vec::new();
        let mut even_temp = Events{ 
                                begin:   "VEVENT".to_string(),
                                dtsart:         date_tz,
                                dtend:          date_tz,
                                dtstamp:        date_tz,
                                uid:            "NULL".to_string(),
                                created:        date_tz,
                                description:    "NULL".to_string(),
                                last_modified:  date_tz,
                                location:       "NULL".to_string(),
                                sequence:       0,
                                status:         "NULL".to_string(),
                                summary:        "NULL".to_string(),
                                transp:         "NULL".to_string(),
                                end:            "VEVENT".to_string()
                            };     
        let mut prodid = String::new();
        let mut version = String::new();
        let mut calscale = String::new();
        let mut method = String::new();
        let mut x_wr_calname = String::new();
        let mut x_wr_timezone = String::new();

        for i in &text_data{
            let key_cal =  i.split(":")
                            .next()
                            .expect("Could not find ':'")
                            .to_string();
            let value_cal =  i.split(":")
                            .last()
                            .expect("Could not find ':'")
                            .to_string();         
            let mut string_key = String::new();
            let num_regex = RegexSet::new(&[
                                            r"PRODID",
                                            r"VERSION",
                                            r"CALSCALE",
                                            r"METHOD",
                                            r"X-WR-CALNAME",
                                            r"X-WR-TIMEZONE",
                                            r"UID",
                                            r"DESCRIPTION",
                                            r"LOCATION",
                                            r"SEQUENCE",
                                            r"STATUS",
                                            r"SUMMARY",
                                            r"TRANSP",
                                            r"DTSTAMP",
                                            r"CREATED",
                                            r"LAST-MODIFIED"
                                            ]).unwrap(); 
            let matches: Vec<_> = num_regex.matches(&key_cal)
                                            .into_iter()
                                            .map(|match_idx| &num_regex.patterns()[match_idx])
                                            .collect();
            if matches.len() > 0{
                string_key = matches[0].to_string();
            }
            else{
                string_key = String::from(key_cal);
            }
            match string_key.as_ref() {
                    "PRODID" => {
                        prodid = value_cal;
                    },
                    "VERSION" => {
                        version = value_cal;
                    },
                    "CALSCALE" => {
                        calscale = value_cal;
                    },
                    "METHOD" => {
                        method = value_cal;
                    },
                    "X-WR-CALNAME" => {
                        x_wr_calname = value_cal;
                    },
                    "X-WR-TIMEZONE" => {
                        x_wr_timezone = value_cal;
                    },
                    "UID" => {
                        even_temp.uid = value_cal;
                    },
                    "DESCRIPTION" => {
                        even_temp.description = value_cal;
                    },
                    "LOCATION" => {
                        even_temp.location = value_cal;
                    },
                    "SEQUENCE" => {
                        even_temp.sequence = value_cal.parse::<u32>().unwrap();
                    },
                    "STATUS" => {
                        even_temp.status = value_cal;
                    },
                    "SUMMARY" => {
                        even_temp.summary = value_cal;
                    },
                    "TRANSP" => {
                        even_temp.transp = value_cal;
                    },
                    "DTSTART" => {
                            let result_obj_aux: Result<DateTime<Utc>, String>;
                            result_obj_aux = convert_datetime(value_cal, "%Y%m%dT%H%M%SZ".to_string());
                            match result_obj_aux{
                                    Ok(val) => {
                                            even_temp.dtsart = val;
                                        },
                                    Err(_) => (),
                            }                        
                    },
                    "DTSTART;VALUE=DATE" => {
                            let aux_date = value_cal+&"T000000Z".to_string();
                            let result_obj_aux: Result<DateTime<Utc>, String>;
                            result_obj_aux = convert_datetime(aux_date, "%Y%m%dT%H%M%SZ".to_string());
                            match result_obj_aux{
                                    Ok(val) => {
                                        even_temp.dtsart = val;
                                        },
                                    Err(_) => (),
                            }
                    },
                    "DTEND" => {
                            let result_obj_aux: Result<DateTime<Utc>, String>;
                            result_obj_aux = convert_datetime(value_cal, "%Y%m%dT%H%M%SZ".to_string());
                            match result_obj_aux{
                                Ok(val) => {
                                        even_temp.dtend = val;
                                        },
                                Err(_) => (),
                            }
                    },
                    "DTEND;VALUE=DATE" => {
                            let time_cal = "T002611Z".to_string(); 
                            let aux_date = value_cal+&time_cal;
                            let result_obj_aux: Result<DateTime<Utc>, String>;
                            result_obj_aux = convert_datetime(aux_date, "%Y%m%dT%H%M%SZ".to_string());
                            match result_obj_aux{
                                Ok(val) => {
                                        even_temp.dtend = val;
                                        },
                                    Err(_) => (),
                            }
                    },
                    "DTSTAMP" => {
                            let result_obj_aux: Result<DateTime<Utc>, String>;
                            result_obj_aux = convert_datetime(value_cal, "%Y%m%dT%H%M%SZ".to_string());
                            match result_obj_aux{
                                    Ok(val) => {
                                        even_temp.dtstamp = val;
                                        },
                                    Err(_) => (),
                                }
                    },
                    "CREATED" => {
                            let result_obj_aux: Result<DateTime<Utc>, String>;
                            result_obj_aux = convert_datetime(value_cal, "%Y%m%dT%H%M%SZ".to_string());
                            match result_obj_aux{
                                Ok(val) => {
                                        even_temp.created = val;
                                        },
                                Err(_) => (),
                            }
                    },
                    "LAST-MODIFIED" => {
                            let result_obj_aux: Result<DateTime<Utc>, String>;
                            result_obj_aux = convert_datetime(value_cal, "%Y%m%dT%H%M%SZ".to_string());
                            match result_obj_aux{
                                    Ok(val) =>{
                                        even_temp.last_modified = val;
                                        },
                                    Err(_) => (),
                            }
                    },
                    "END" if value_cal == "VEVENT" => {  
                            let even_clone = even_temp.clone();   
                            struct_even.push(even_clone);
                        },
                        _ => (),
                    }
                    
                }
                Calendar{
                    begin:          "VCALENDAR".to_string(),
                    prodid:         prodid,
                    version:        version,
                    calscale:       calscale,
                    method:         method,
                    x_wr_calname:   x_wr_calname,
                    x_wr_timezone:  x_wr_timezone,
                    events:         struct_even,
                    end:            "VCALENDAR".to_string()
                }
            } 
        }