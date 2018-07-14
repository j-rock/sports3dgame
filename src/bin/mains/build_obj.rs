use game::{
    app::AppRunner,
    file,
    entity::{
        Ball,
    },
};

pub fn _main() {
    let _app = AppRunner::new();
//    let athlete = Athlete::new().unwrap();
    let ball = Ball::new().unwrap();
    let out = file::util::resource_path("temp", "ball");
    ball.sprite.write_to_objs(out.as_str()).unwrap();
}
