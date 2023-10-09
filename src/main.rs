use rust::examples::{conway::Conway, shapes::{Rectangle, Shape}};
fn main() {
    let mut state = vec![];
    for i in 0..25 {
        let mut row = vec![];
        for j in 0..35 {
            row.push((i * j * 2 / (i + j + 3)) % 3 == 0);
        }
        state.push(row);
    }

    let r = Rectangle {
        width: 10.0,
        length: 12.0,
    };

    println!("{}", r.area());

    // let c = Conway::new(state);
    let c = Conway::gosper_glider_gun();
    c.print_loop();
}
