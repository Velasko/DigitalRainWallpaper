import QtQuick 2.6
import QtQuick.Window 2.0
// Import our Rust classes
import Greeter 1.0

Window {
    visible: true
    // Instantiate the rust struct
    Greeter {
        id: greeter;
        // Set a property
        name: "Worldo"
    }
    Text {
        anchors.centerIn: parent
        // Call a method
        text: greeter.compute_greetings("hello")
    }
}
