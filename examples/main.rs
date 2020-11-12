use std::convert::TryInto;
use std::io::{Seek, SeekFrom, Write};
use std::os::unix::io::IntoRawFd;
use std::time::Instant;
use tempfile::tempfile;
use wayland_client::protocol::wl_seat::WlSeat;
use wayland_client::Display;
use wayland_client::EventQueue;
use wayland_client::Main;
use zwp_virtual_keyboard::virtual_keyboard_unstable_v1::zwp_virtual_keyboard_manager_v1::ZwpVirtualKeyboardManagerV1;
use zwp_virtual_keyboard::virtual_keyboard_unstable_v1::zwp_virtual_keyboard_v1::ZwpVirtualKeyboardV1;

mod keymap;
mod wayland;

#[derive(Debug, Clone)]
pub enum SubmitError {
    /// Virtual keyboard proxy was dropped and is no longer alive
    NotAlive,
    InvalidKeycode,
}

enum KeyMotion {
    Press = 1,
    Release = 0,
}

#[derive(Debug, Clone, Copy)]
enum KeyState {
    Pressed = 1,
    Released = 0,
}

pub struct VKService {
    display: Display,
    event_queue: EventQueue, // Preventing event_queue from being dropped
    base_time: std::time::Instant,
    shift_state: KeyState,
    virtual_keyboard: Main<ZwpVirtualKeyboardV1>,
}

impl VKService {
    pub fn new(
        display: Display,
        event_queue: EventQueue,
        seat: &WlSeat,
        vk_mgr: Main<ZwpVirtualKeyboardManagerV1>,
    ) -> VKService {
        let base_time = Instant::now();
        let shift_state = KeyState::Released;
        let virtual_keyboard = vk_mgr.create_virtual_keyboard(&seat);
        let vk_service = VKService {
            display,
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
            memmap::MmapOptions::new()
                .map_mut(&keymap_file)
                .expect("Could not access data from memory mapped file")
        };
        data[..src.len()].copy_from_slice(src.as_bytes());
        let keymap_raw_fd = keymap_file.into_raw_fd();
        self.virtual_keyboard
            .keymap(1, keymap_raw_fd, keymap_size_u32);
    }

    fn get_time(&self) -> u32 {
        let duration = self.base_time.elapsed();
        let time = duration.as_millis();
        time.try_into().unwrap()
    }

    // Press and then release the key
    pub fn submit_keycode(&mut self, keycode: &str) -> Result<(), SubmitError> {
        if let Some(keycode) = input_event_codes_hashmap::KEY.get(keycode) {
            let press_result = self.send_key(*keycode, KeyMotion::Press);
            self.event_queue
                .sync_roundtrip(&mut (), |raw_event, _, _| {
                    println!("Unhandled Event: {:?}", raw_event)
                })
                .unwrap();
            if press_result.is_ok() {
                // Make the key press last two seconds
                std::thread::sleep(std::time::Duration::from_millis(2000));
                let result = self.send_key(*keycode, KeyMotion::Release);
                self.event_queue
                    .sync_roundtrip(&mut (), |raw_event, _, _| {
                        println!("Unhandled Event: {:?}", raw_event)
                    })
                    .unwrap();
                result
            } else {
                press_result
            }
        } else {
            Err(SubmitError::InvalidKeycode)
        }
    }

    fn send_key(&self, keycode: u32, keymotion: KeyMotion) -> Result<(), SubmitError> {
        let time = self.get_time();
        println!("time: {}, keycode: {}", time, keycode);
        if self.virtual_keyboard.as_ref().is_alive() {
            self.virtual_keyboard.key(time, keycode, keymotion as u32);
            Ok(())
        } else {
            Err(SubmitError::NotAlive)
        }
    }

    pub fn toggle_shift(&mut self) -> Result<(), SubmitError> {
        match self.shift_state {
            KeyState::Pressed => self.shift_state = KeyState::Released,
            KeyState::Released => self.shift_state = KeyState::Pressed,
        }
        if self.virtual_keyboard.as_ref().is_alive() {
            self.virtual_keyboard.modifiers(
                self.shift_state as u32, //mods_depressed,
                0,                       //mods_latched
                0,                       //mods_locked
                0,                       //group
            );
            Ok(())
        } else {
            Err(SubmitError::NotAlive)
        }
    }
}

fn main() {
    let (display, event_queue, seat, vk_mgr) = wayland::init_wayland();
    let mut vk_service = VKService::new(display, event_queue, &seat, vk_mgr);
    let submission_result = vk_service.submit_keycode("X");
    if submission_result.is_err() {
        println!("Error: {:?}", submission_result);
    };
}
