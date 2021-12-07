#[macro_use]
extern crate rocket;
use rand::Rng;

#[get("/<columns>/<rows>")]
fn run(columns: u16, rows: u16) -> String {
    let mut form = String::with_capacity(((columns * rows * 2) + (columns * 3) + 2) as usize);
    form.push('{');
    for _ in 0..columns {
        form.push_str("\n{");
        for _ in 0..rows {
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
    println!("{}", form.chars().count());
    return format!("{}", form);
}
#[get("/")]
fn home() -> String {
    format!("Use the form columns/row in the address")
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![run, home])
}
