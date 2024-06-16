// use watcher::watchlib::watch;
//
// fn main() {
//     watch::start_watch("/home/Downloads");
// }

use watcher::parser::parser;

fn main() {
    let result = parser::parse().unwrap();
}
