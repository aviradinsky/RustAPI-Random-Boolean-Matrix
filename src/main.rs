#[macro_use] extern crate rocket;
use rand::Rng;

#[get("/<columns>/<rows>")]
fn run(columns: u16, rows: u16) -> String {
    let mut form = String::from("{");
    for _ in 0..columns{
        form.push_str("\n{");
        for _ in 0..rows{
            if rand::thread_rng().gen_weighted_bool(2){
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
    return format!("{}",form);
}
#[get("/")]
fn home() -> String {
    format!("Use the form columns/row in the address")
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![run,home])
}