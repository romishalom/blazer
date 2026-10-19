use blazer::spinner::Spinner;
use blazer::symbols::frames::ARROW;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut spinner = Spinner::new("Fetching data...").with_frames(ARROW);
    spinner.start();
    sleep(Duration::from_secs(2));
    spinner.stop("Fetched successfully!");
}