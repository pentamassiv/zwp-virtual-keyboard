use wayland_client::{protocol::wl_seat::WlSeat, Display, EventQueue, GlobalManager, Main, Proxy};
use zwp_virtual_keyboard::virtual_keyboard_unstable_v1::zwp_virtual_keyboard_manager_v1::ZwpVirtualKeyboardManagerV1;

fn get_wl_global_mgr(display: Display) -> (EventQueue, GlobalManager) {
    // Create the event queue
    let mut event_queue = display.create_event_queue();
    // Attach the display
    let attached_display = display.attach(event_queue.token());

    let global_mgr = GlobalManager::new(&attached_display);

    // sync_roundtrip is a special kind of dispatching for the event queue.
    // Rather than just blocking once waiting for replies, it'll block
    // in a loop until the server has signalled that it has processed and
    // replied accordingly to all requests previously sent by the client.
    //
    // In our case, this allows us to be sure that after this call returns,
    // we have received the full list of globals.
    event_queue
        .sync_roundtrip(
            // we don't use a global state for this example
            &mut (),
            // The only object that can receive events is the WlRegistry, and the
            // GlobalManager already takes care of assigning it to a callback, so
            // we cannot receive orphan events at this point
            |_, _, _| println!("Event received that was not handled"), // For testing
                                                                       //|_, _, _| unreachable!(), // Original
        )
        .unwrap();
    (event_queue, global_mgr)
}

pub fn init_wayland() -> (
    Display,
    EventQueue,
    WlSeat,
    Main<ZwpVirtualKeyboardManagerV1>,
) {
    let display = Display::connect_to_env()
        .or_else(|_| Display::connect_to_name("wayland-0"))
        .unwrap();
    let (mut event_queue, global_mgr) = get_wl_global_mgr(display.clone());
    let seat = global_mgr.instantiate_exact::<WlSeat>(7).unwrap();
    let seat: WlSeat = WlSeat::from(seat.as_ref().clone());
    let vk_mgr = global_mgr
        .instantiate_exact::<ZwpVirtualKeyboardManagerV1>(1)
        .expect("Error: Your compositor does not understand the virtual_keyboard protocol!");
    (display, event_queue, seat, vk_mgr)
}
