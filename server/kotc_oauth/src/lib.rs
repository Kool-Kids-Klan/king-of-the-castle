extern crate google_signin;

fn main() {
    let mut client = google_signin::Client::new();
    client.audiences.push("229405536082-o0p730oresk0eeprtm1j9p27523thc47.apps.googleusercontent.com".to_string()); // required
}
