fn main() {
    run_winit();
}

fn run_winit() {
    let mut instance = sjvi::winit::Instance::new();
    let _id = instance.create_display();

    while instance.try_update() {}
}

#[allow(dead_code)]
fn run_glow() {}
