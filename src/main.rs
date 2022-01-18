#[macro_use]
extern crate rocket;
use rand::Rng;

#[get("/<columns>/<rows>")]
fn run(columns: u16, rows: u16) -> String {
    let mut form = String::with_capacity((columns * rows * 2 + rows * 3 + 2) as usize);
    form.push('{');
    for _ in 0..rows {
        form.push_str("\n{");
        for _ in 0..columns {
            if rand::thread_rng().gen_weighted_bool(2) {
                form.push('1');
            } else {
                form.push('0');
            }
            form.push(',');
        }
        form.pop();
        form.push_str("},")
    }
    form.pop();
    form.push_str("\n}");
    form
}
#[get("/")]
fn home() -> String {
    "Use the form columns/row in the address".to_string()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![run, home])
}
