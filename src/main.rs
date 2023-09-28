mod picross;

fn main() {
    let all_keys = picross::AllKeys::new("mouse");
    all_keys.show();
}
