
/*
* Protocol: USB HID 
*   - 1000 packets per second about mouse state
*   - When mouse moves sends a stream of data packets with current X Y of cursor.
*   - When button pressed sends a packet with state pressed or released.
*   - When scroll wheel pressed sends packet indicating direction and distance of rotation.

+------+------+------+------+------+
|  ID  |  X   |  Y   |  B1  |  B2  |
+------+------+------+------+------+
ID:         A single byte that identifies the packet as a mouse report.
X and Y:    Two bytes that contain the X and Y position of the cursor, respectively. These values are typically sent as signed integers and are measured in counts, which can be converted to coordinates on the screen using the resolution of the sensor.
B1 and B2:  Two bytes that contain the state of the mouse buttons. These values are typically sent as bit fields, where a set bit indicates that the corresponding button is pressed.

Considerations:
- Libs: libusb, winusb, hidapi.
- Methods to handle converstion from struct to packet.
    - Also endianness, other device may have different endianness

*
* */

pub struct MousePacket {
    pub id: u8,
    pub x: i16,
    pub y: i16,
    pub b1: u8,
    pub b2: u8,
}


