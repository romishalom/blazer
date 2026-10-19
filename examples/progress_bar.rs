use blazer::progress::ProgressBar;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut bar = ProgressBar::new(40).with_label("Processing");
    for i in 0..=40 {
        bar.set_position(i);
        bar.draw();
        sleep(Duration::from_millis(30));
    }
    bar.finish_with_message("All done!");
}