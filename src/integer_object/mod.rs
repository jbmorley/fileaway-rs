mod imp;

use glib::Object;
use gtk::glib;

// ANCHOR: integer_object
glib::wrapper! {
    pub struct IntegerObject(ObjectSubclass<imp::IntegerObject>);
}

impl IntegerObject {
    pub fn new(number: i32) -> Self {
        return Object::builder().property("number", number).build();
    }
}
// ANCHOR_END: integer_object
