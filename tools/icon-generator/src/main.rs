mod favicon;
mod help;
mod missing;
mod placeholder;
mod unavailable;

fn main() {
    favicon::build();
    unavailable::build();
    missing::build();
    placeholder::build();
}
