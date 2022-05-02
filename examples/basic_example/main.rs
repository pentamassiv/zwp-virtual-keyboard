use std::convert::AsRef;
use std::convert::TryInto;
use std::io::{Seek, SeekFrom, Write};
use std::os::unix::io::IntoRawFd;
use std::time::{Duration, Instant};
use tempfile::tempfile;
use wayland_client::{protocol::wl_seat::WlSeat, EventQueue, Main};
use zwp_virtual_keyboard::virtual_keyboard_unstable_v1::zwp_virtual_keyboard_manager_v1::ZwpVirtualKeyboardManagerV1;
use zwp_virtual_keyboard::virtual_keyboard_unstable_v1::zwp_virtual_keyboard_v1::ZwpVirtualKeyboardV1;

mod keymap;
mod wayland;

type KeyCode = u32;

#[derive(Debug, Clone)]
pub enum SubmitError {
    /// Virtual keyboard proxy was dropped and is no longer alive
    NotAlive,
}

#[derive(Debug, Clone, Copy)]
enum KeyState {
    Pressed = 1,
    Released = 0,
}

pub struct VKService {
    event_queue: EventQueue, // Preventing event_queue from being dropped
    base_time: std::time::Instant,
    shift_state: KeyState,
    virtual_keyboard: Main<ZwpVirtualKeyboardV1>,
}

impl VKService {
    pub fn new(
        event_queue: EventQueue,
        seat: &WlSeat,
        vk_mgr: Main<ZwpVirtualKeyboardManagerV1>,
    ) -> VKService {
        let base_time = Instant::now();
        let shift_state = KeyState::Released;
        let virtual_keyboard = vk_mgr.create_virtual_keyboard(&seat);
        let vk_service = VKService {
            event_queue,
            base_time,
            shift_state,
            virtual_keyboard,
        };
        vk_service.init_virtual_keyboard();
        vk_service
    }

    fn init_virtual_keyboard(&self) {
        println!("keyboard initialized");
        let src = keymap::KEYMAP;
        let keymap_size = keymap::KEYMAP.len();
        let keymap_size_u32: u32 = keymap_size.try_into().unwrap(); // Convert it from usize to u32, panics if it is not possible
        let keymap_size_u64: u64 = keymap_size.try_into().unwrap(); // Convert it from usize to u64, panics if it is not possible
        let mut keymap_file = tempfile().expect("Unable to create tempfile");
        // Allocate space in the file first
        keymap_file.seek(SeekFrom::Start(keymap_size_u64)).unwrap();
        keymap_file.write_all(&[0]).unwrap();
        keymap_file.seek(SeekFrom::Start(0)).unwrap();
        let mut data = unsafe {
            memmap2::MmapOptions::new()
                .map_mut(&keymap_file)
                .expect("Could not access data from memory mapped file")
        };
        data[..src.len()].copy_from_slice(src.as_bytes());
        let keymap_raw_fd = keymap_file.into_raw_fd();
        self.virtual_keyboard
            .keymap(1, keymap_raw_fd, keymap_size_u32);
    }

    fn get_duration(&mut self) -> u32 {
        let duration = self.base_time.elapsed();
        let duration = duration.as_millis();
        if let Ok(duration) = duration.try_into() {
            duration
        } else {
            // Reset the base time if it was too big for a u32
            self.base_time = Instant::now();
            self.get_duration()
        }
    }

    fn send_event(&mut self) {
        self.event_queue
            .sync_roundtrip(&mut (), |raw_event, _, _| {
                println!("Unhandled Event: {:?}", raw_event)
            })
            .unwrap();
    }

    // Press and then release the key
    pub fn long_press_keycode(&mut self, keycode: KeyCode) -> Result<(), SubmitError> {
        let press_result = self.send_key(keycode, KeyState::Pressed);
        self.send_event();
        if press_result.is_ok() {
            // Make the key press last two seconds
            std::thread::sleep(Duration::from_millis(2000));
            let result = self.send_key(keycode, KeyState::Released);
            self.send_event();
            result
        } else {
            press_result
        }
    }

    fn send_key(
        &mut self,
        keycode: KeyCode,
        desired_key_state: KeyState,
    ) -> Result<(), SubmitError> {
        let time = self.get_duration();
        println!("time: {}, keycode: {}", time, keycode);
        if self.virtual_keyboard.as_ref().is_alive() {
            self.virtual_keyboard
                .key(time, keycode, desired_key_state as u32);
            Ok(())
        } else {
            Err(SubmitError::NotAlive)
        }
    }

    pub fn toggle_shift(&mut self) -> Result<(), SubmitError> {
        let shift_key = input_event_codes::KEY_X!();
        let (mut mods_depressed, mut _mods_latched, mut mods_locked, group) = (0, 0, 0, 0);

        match self.shift_state {
            KeyState::Pressed => {
                self.shift_state = KeyState::Released;
                mods_depressed = shift_key;
            }
            KeyState::Released => {
                self.shift_state = KeyState::Pressed;
                mods_locked = shift_key;
            }
        }
        if self.virtual_keyboard.as_ref().is_alive() {
            self.virtual_keyboard.modifiers(
                mods_depressed, //mods_depressed,
                _mods_latched,  //mods_latched
                mods_locked,    //mods_locked
                group,          //group
            );
            Ok(())
        } else {
            Err(SubmitError::NotAlive)
        }
    }
}

fn main() {
    let (_, event_queue, seat, vk_mgr) = wayland::init_wayland();
    let mut vk_service = VKService::new(event_queue, &seat, vk_mgr);
    let key = input_event_codes::KEY_X!();

    // Long press x
    let submission_result = vk_service.long_press_keycode(key);
    if submission_result.is_err() {
        println!("Error: {:?}", submission_result);
    };
    println!("Long press done");

    // Toggle shift and long press x
    let submission_result = vk_service.toggle_shift();
    if submission_result.is_err() {
        println!("Error: {:?}", submission_result);
    };
    let submission_result = vk_service.long_press_keycode(key);
    if submission_result.is_err() {
        println!("Error: {:?}", submission_result);
    };
    println!("First toggle shift and long press x");

    // Toggle shift and long press x
    let submission_result = vk_service.toggle_shift();
    if submission_result.is_err() {
        println!("Error: {:?}", submission_result);
    };
    let submission_result = vk_service.long_press_keycode(key);
    if submission_result.is_err() {
        println!("Error: {:?}", submission_result);
    };
    println!("Second toggle shift and long press x");
}
