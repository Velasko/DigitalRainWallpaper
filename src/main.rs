use cstr::cstr;
use qmetaobject::prelude::*;
use qmetaobject::QUrl;

// The `QObject` custom derive macro allows to expose a class to Qt and QML
#[derive(QObject, Default)]
struct Greeter {
    // Specify the base class with the qt_base_class macro
    base: qt_base_class!(trait QObject),
    // Declare `name` as a property usable from Qt
    name: qt_property!(QString; NOTIFY name_changed),
    // Declare a signal
    name_changed: qt_signal!(),
    // And even a slot
    compute_greetings: qt_method!(
        fn compute_greetings(&self, verb: String) -> QString {
            format!("{} {}", verb, self.name.to_string()).into()
        }
    ),
}

qrc!(root_qml,
    "" {
        "ui/main.qml" as "main.qml",
    }
);

fn main2() {
    // Register the `Greeter` struct to QML
    qml_register_type::<Greeter>(cstr!("Greeter"), 1, 0, cstr!("Greeter"));
    // Create a QML engine from rust
    let mut engine = QmlEngine::new();

    // Load Resource
    root_qml();

    engine.load_url(QUrl::from(QString::from("qrc:///main.qml")));
    engine.exec();
}

mod canvas;
mod common;
mod drop;

extern crate structopt;
use crate::structopt::StructOpt;

use crate::canvas::canvas::*;
use crate::canvas::layer::*;

fn main() {
    let can_par = CanvasParser::from_args();
    // println!("{:?}\n", can_par);

    let can = Canvas::<Layer<u8>>::parse(can_par);
    println!("{:?}", can);

    println!("\n## Code end ##");
}
