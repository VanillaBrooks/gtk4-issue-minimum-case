use gtk::prelude::*;

use gtk::Application;
use gtk::ApplicationWindow;
use gtk::ListBox;
use gtk::ListBoxRow;
use gtk::MediaFile;
use gtk::Picture;

const SIZE : i32 = 400;

fn main() {
    let application = Application::new(Some("gif.example"), Default::default());

    application.connect_activate(move |app: &Application| {
        //
        build_ui(app);
    });

    application.run();
}

/// put a MediaFile inside a Picture thats attached directly to the application window
///
/// Behavior: The animation "twitches" through the first few frames without playing the
/// full duration
fn build_ui(application: &Application) {
    let window = ApplicationWindow::builder()
        .application(application)
        .title("gif-display-example")
        .build();

    let media = MediaFile::for_filename("catJAM.gif");
    media.set_loop(true);
    media.set_playing(true);

    let picture = Picture::builder()
        .paintable(&media)
        .height_request(SIZE)
        .width_request(SIZE)
        .build();

    window.set_child(Some(&picture));

    window.show()
}
