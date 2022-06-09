fn main() {
    let mut instance = sjvi::Instance::new();
    let id0 = instance.create_display();
    let id1 = instance.create_display();

    while instance.try_update() {
        if let Some(_display) = instance.try_get_display(id0) {
            // do something
        }

        if let Some(_display) = instance.try_get_display(id1) {
            // do something
        }
    }
}
