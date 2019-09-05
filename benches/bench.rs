extern crate test;

#[bench]
fn export_ics(bencher: &mut test::Bencher) {
    use crate::Calendar;
    let ical = Calendar::new("https://gist.githubusercontent.com/DeMarko/6142417/raw/1cd301a5917141524b712f92c2e955e86a1add19/sample.ics");
    bencher.iter(|| ical.export_ics("/tmp/1"))
}

#[bench]
fn export_writer(bencher: &mut test::Bencher) {
    use crate::Calendar;
    let ical = Calendar::new("https://gist.githubusercontent.com/DeMarko/6142417/raw/1cd301a5917141524b712f92c2e955e86a1add19/sample.ics");
    bencher.iter(|| {
        let mut file = std::fs::File::create("/tmp/2").unwrap();
        ical.export_to(&mut file)
    })
}
