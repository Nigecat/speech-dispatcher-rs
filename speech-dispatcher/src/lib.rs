#![allow(non_upper_case_globals)]

use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::marker::Send;
use std::sync::Mutex;

use lazy_static::lazy_static;
use speech_dispatcher_sys::*;

#[derive(Clone, Copy, Debug)]
#[repr(u32)]
pub enum Mode {
    Single = SPDConnectionMode::SPD_MODE_SINGLE,
    Threaded = SPDConnectionMode::SPD_MODE_THREADED,
}

#[derive(Clone, Copy, Debug)]
#[repr(u32)]
pub enum Priority {
    Important = SPDPriority::SPD_IMPORTANT,
    Message = SPDPriority::SPD_MESSAGE,
    Text = SPDPriority::SPD_TEXT,
    Notification = SPDPriority::SPD_NOTIFICATION,
    Progress = SPDPriority::SPD_PROGRESS,
}

#[derive(Clone, Copy, Debug)]
#[repr(u32)]
pub enum VoiceType {
    Male1 = SPDVoiceType::SPD_MALE1,
    Male2 = SPDVoiceType::SPD_MALE2,
    Male3 = SPDVoiceType::SPD_MALE3,
    Female1 = SPDVoiceType::SPD_FEMALE1,
    Female2 = SPDVoiceType::SPD_FEMALE2,
    Female3 = SPDVoiceType::SPD_FEMALE3,
    ChildMale = SPDVoiceType::SPD_CHILD_MALE,
    ChildFemale = SPDVoiceType::SPD_CHILD_FEMALE,
}

#[derive(Clone, Debug)]
pub struct Connection(pub *mut SPDConnection, u64);

pub type Address = SPDConnectionAddress;

#[derive(Clone, Copy, Debug)]
#[repr(u32)]
pub enum DataMode {
    Text = SPDDataMode::SPD_DATA_TEXT,
    SSML = SPDDataMode::SPD_DATA_SSML,
}

#[derive(Clone, Copy, Debug)]
#[repr(u32)]
pub enum Notification {
    Begin = SPDNotification::SPD_BEGIN,
    End = SPDNotification::SPD_END,
    IndexMarks = SPDNotification::SPD_INDEX_MARKS,
    Cancel = SPDNotification::SPD_CANCEL,
    Pause = SPDNotification::SPD_PAUSE,
    Resume = SPDNotification::SPD_RESUME,
    All = SPDNotification::SPD_ALL,
}

#[derive(Clone, Copy, Debug)]
#[repr(u32)]
pub enum Punctuation {
    All = SPDPunctuation::SPD_PUNCT_ALL,
    None = SPDPunctuation::SPD_PUNCT_NONE,
    Some = SPDPunctuation::SPD_PUNCT_SOME,
}

#[derive(Clone, Copy, Debug)]
#[repr(u32)]
pub enum CapitalLetters {
    None = SPDCapitalLetters::SPD_CAP_NONE,
    Spell = SPDCapitalLetters::SPD_CAP_SPELL,
    Icon = SPDCapitalLetters::SPD_CAP_ICON,
}

fn i32_to_bool(v: i32) -> bool {
    v == 1
}

#[derive(Default)]
struct Callbacks {
    begin: Option<Box<dyn FnMut(u64, u64)>>,
    end: Option<Box<dyn FnMut(u64, u64)>>,
    index_mark: Option<Box<dyn FnMut(u64, u64, String)>>,
    cancel: Option<Box<dyn FnMut(u64, u64)>>,
    pause: Option<Box<dyn FnMut(u64, u64)>>,
    resume: Option<Box<dyn FnMut(u64, u64)>>,
}

unsafe impl Send for Callbacks {}

unsafe impl Sync for Callbacks {}

lazy_static! {
    static ref callbacks: Mutex<HashMap<u64, Callbacks>> = {
        let m = HashMap::new();
        Mutex::new(m)
    };
}

unsafe extern "C" fn cb(msg_id: u64, client_id: u64, state: u32) {
    let state = match state {
        SPDNotificationType_SPD_EVENT_BEGIN => Notification::Begin,
        SPDNotificationType_SPD_EVENT_END => Notification::End,
        SPDNotificationType_SPD_EVENT_CANCEL => Notification::Cancel,
        SPDNotificationType_SPD_EVENT_PAUSE => Notification::Pause,
        SPDNotificationType_SPD_EVENT_RESUME => Notification::Resume,
        _ => panic!("Unknown notification received in callback: {}", state),
    };
    if let Some(c) = callbacks.lock().unwrap().get_mut(&client_id) {
        let f = match state {
            Notification::Begin => &mut c.begin,
            Notification::End => &mut c.end,
            Notification::Cancel => &mut c.cancel,
            Notification::Pause => &mut c.pause,
            Notification::Resume => &mut c.resume,
            _ => panic!("Unknown notification type"),
        };
        if let Some(f) = f.as_mut() {
            f(msg_id, client_id);
        }
    }
}

unsafe extern "C" fn cb_im(msg_id: u64, client_id: u64, state: u32, index_mark: *mut i8) {
    let index_mark = CStr::from_ptr(index_mark);
    let index_mark = index_mark.to_string_lossy().to_string();
    let state = match state {
        SPDNotificationType_SPD_EVENT_INDEX_MARK => Notification::IndexMarks,
        _ => panic!("Unknown notification received in IM callback: {}", state),
    };
    if let Some(c) = callbacks.lock().unwrap().get_mut(&client_id) {
        let f = match state {
            Notification::IndexMarks => &mut c.index_mark,
            _ => panic!("Unknown notification type"),
        };
        if let Some(f) = f.as_mut() {
            f(msg_id, client_id, index_mark);
        }
    }
}

impl Connection {
    pub fn open<S: Into<String>>(
        client_name: S,
        connection_name: S,
        user_name: S,
        mode: Mode,
    ) -> Self {
        let clientname = CString::new(client_name.into()).unwrap();
        let connectionname = CString::new(connection_name.into()).unwrap();
        let username = CString::new(user_name.into()).unwrap();
        let connection = unsafe {
            let c = spd_open(
                clientname.as_ptr(),
                connectionname.as_ptr(),
                username.as_ptr(),
                mode as u32,
            );
            Self::setup_connection(c)
        };
        let mut c = Self(connection, 0);
        c.setup();
        c
    }

    pub unsafe fn open2<S: Into<String>>(
        client_name: S,
        connection_name: S,
        user_name: S,
        mode: Mode,
        address: *mut Address,
        autospawn: bool,
    ) -> Self {
        let auto_spawn = if autospawn { 1 } else { 0 };
        let error_result = vec![CString::new("").unwrap().into_raw()].as_mut_ptr();
        let clientname = CString::new(client_name.into()).unwrap();
        let connectionname = CString::new(connection_name.into()).unwrap();
        let username = CString::new(user_name.into()).unwrap();
        let connection = {
            let c = spd_open2(
                clientname.as_ptr(),
                connectionname.as_ptr(),
                username.as_ptr(),
                mode as u32,
                address,
                auto_spawn,
                error_result,
            );
            Self::setup_connection(c)
        };
        let mut c = Self(connection, 0);
        c.setup();
        c
    }

    unsafe fn setup_connection(c: *mut SPDConnection) -> *mut SPDConnection {
        (*c).callback_begin = Some(cb);
        (*c).callback_end = Some(cb);
        (*c).callback_cancel = Some(cb);
        (*c).callback_pause = Some(cb);
        (*c).callback_resume = Some(cb);
        (*c).callback_im = Some(cb_im);
        c
    }

    fn setup(&mut self) {
        let client_id = self.send_data("HISTORY GET CLIENT_ID\r\n", true);
        if let Some(client_id) = client_id {
            let client_id: Vec<&str> = client_id.split("\r\n").collect();
            let client_id = client_id.get(0);
            if let Some(client_id) = client_id {
                let client_id: Vec<&str> = client_id.split("-").collect();
                if let Some(client_id) = client_id.get(1) {
                    if let Ok(client_id) = client_id.parse::<u64>() {
                        self.1 = client_id;
                    }
                }
            }
        }
        callbacks.lock().unwrap().insert(self.1, Default::default());
        self.set_notification_on(Notification::All);
    }

    pub fn close(&self) {
        unsafe { spd_close(self.0) };
    }

    pub fn say<S: Into<String>>(&self, priority: Priority, text: S) -> Option<u64> {
        let text: String = text.into();
        let param = CString::new(text).unwrap();
        let rv = unsafe { spd_say(self.0, priority as u32, param.as_ptr()) };
        if rv != -1 {
            Some(rv as u64)
        } else {
            None
        }
    }

    pub fn sayf<S: Into<String>>(&self, priority: Priority, format: S) -> Option<i32> {
        let format: String = format.into();
        let param = CString::new(format).unwrap();
        let rv = unsafe { spd_sayf(self.0, priority as u32, param.as_ptr()) };
        if rv != -1 {
            Some(rv)
        } else {
            None
        }
    }

    pub fn stop(&self) -> bool {
        let v = unsafe { spd_stop(self.0) };
        i32_to_bool(v)
    }

    pub fn stop_all(&self) -> bool {
        let v = unsafe { spd_stop_all(self.0) };
        i32_to_bool(v)
    }

    pub fn stop_uid(&self, target_uid: i32) -> bool {
        let v = unsafe { spd_stop_uid(self.0, target_uid) };
        i32_to_bool(v)
    }

    pub fn cancel(&self) -> bool {
        let v = unsafe { spd_cancel(self.0) };
        i32_to_bool(v)
    }

    pub fn cancel_all(&self) -> bool {
        let v = unsafe { spd_cancel_all(self.0) };
        i32_to_bool(v)
    }

    pub fn cancel_uid(&self, target_uid: i32) -> bool {
        let v = unsafe { spd_cancel_uid(self.0, target_uid) };
        i32_to_bool(v)
    }

    pub fn pause(&self) -> bool {
        let v = unsafe { spd_pause(self.0) };
        i32_to_bool(v)
    }

    pub fn pause_all(&self) -> bool {
        let v = unsafe { spd_pause_all(self.0) };
        i32_to_bool(v)
    }

    pub fn pause_uid(&self, target_uid: i32) -> bool {
        let v = unsafe { spd_pause_uid(self.0, target_uid) };
        i32_to_bool(v)
    }

    pub fn resume(&self) -> bool {
        let v = unsafe { spd_resume(self.0) };
        i32_to_bool(v)
    }

    pub fn resume_all(&self) -> bool {
        let v = unsafe { spd_resume_all(self.0) };
        i32_to_bool(v)
    }

    pub fn resume_uid(&self, target_uid: i32) -> bool {
        let v = unsafe { spd_resume_uid(self.0, target_uid) };
        i32_to_bool(v)
    }

    pub fn key<S: Into<String>>(&self, priority: Priority, key_name: S) -> bool {
        let param = CString::new(key_name.into()).unwrap();
        let v = unsafe { spd_key(self.0, priority as u32, param.as_ptr()) };
        i32_to_bool(v)
    }

    pub fn char<S: Into<String>>(&self, priority: Priority, char: S) -> bool {
        let param = CString::new(char.into()).unwrap();
        let v = unsafe { spd_char(self.0, priority as u32, param.as_ptr()) };
        i32_to_bool(v)
    }

    pub fn wchar(&self, priority: Priority, wchar: i32) -> bool {
        let v = unsafe { spd_wchar(self.0, priority as u32, wchar) };
        i32_to_bool(v)
    }

    pub fn sound_icon<S: Into<String>>(&self, priority: Priority, icon_name: S) -> bool {
        let param = CString::new(icon_name.into()).unwrap();
        let v = unsafe { spd_char(self.0, priority as u32, param.as_ptr()) };
        i32_to_bool(v)
    }

    pub fn set_voice_type(&self, voice_type: VoiceType) -> bool {
        let v = unsafe { spd_set_voice_type(self.0, voice_type as u32) };
        i32_to_bool(v)
    }

    pub fn set_voice_type_all(&self, voice_type: VoiceType) -> bool {
        let v = unsafe { spd_set_voice_type_all(self.0, voice_type as u32) };
        i32_to_bool(v)
    }

    pub fn set_voice_type_uid(&self, voice_type: VoiceType, target_uid: u32) -> bool {
        let v = unsafe { spd_set_voice_type_uid(self.0, voice_type as u32, target_uid) };
        i32_to_bool(v)
    }

    pub fn get_voice_type(&self) -> VoiceType {
        let v = unsafe { spd_get_voice_type(self.0) };
        match v {
            SPDVoiceType::SPD_MALE1 => VoiceType::Male1,
            SPDVoiceType::SPD_MALE2 => VoiceType::Male2,
            SPDVoiceType::SPD_MALE3 => VoiceType::Male3,
            SPDVoiceType::SPD_FEMALE1 => VoiceType::Female1,
            SPDVoiceType::SPD_FEMALE2 => VoiceType::Female2,
            SPDVoiceType::SPD_FEMALE3 => VoiceType::Female3,
            SPDVoiceType::SPD_CHILD_MALE => VoiceType::ChildMale,
            SPDVoiceType::SPD_CHILD_FEMALE => VoiceType::ChildFemale,
            _ => panic!("Invalid voice type"),
        }
    }

    pub fn set_synthesis_voice<S: Into<String>>(&self, voice_name: S) -> bool {
        let param = CString::new(voice_name.into()).unwrap();
        let v = unsafe { spd_set_synthesis_voice(self.0, param.as_ptr()) };
        i32_to_bool(v)
    }

    pub fn set_synthesis_voice_all<S: Into<String>>(&self, voice_name: S) -> bool {
        let param = CString::new(voice_name.into()).unwrap();
        let v = unsafe { spd_set_synthesis_voice_all(self.0, param.as_ptr()) };
        i32_to_bool(v)
    }

    pub fn set_synthesis_voice_uid<S: Into<String>>(&self, voice_name: S, target_uid: u32) -> bool {
        let param = CString::new(voice_name.into()).unwrap();
        let v = unsafe { spd_set_synthesis_voice_uid(self.0, param.as_ptr(), target_uid) };
        i32_to_bool(v)
    }

    pub fn set_data_mode(&self, mode: DataMode) -> bool {
        let v = unsafe { spd_set_data_mode(self.0, mode as u32) };
        i32_to_bool(v)
    }

    pub fn set_notification_on(&self, notification: Notification) -> bool {
        let v = unsafe { spd_set_notification_on(self.0, notification as u32) };
        i32_to_bool(v)
    }

    pub fn set_notification_off(&self, notification: Notification) -> bool {
        let v = unsafe { spd_set_notification_off(self.0, notification as u32) };
        i32_to_bool(v)
    }

    pub fn set_notification<S: Into<String>>(&self, notification: Notification, state: S) -> bool {
        let param = CString::new(state.into()).unwrap();
        let v = unsafe { spd_set_notification(self.0, notification as u32, param.as_ptr()) };
        i32_to_bool(v)
    }

    pub fn set_voice_rate(&self, rate: i32) -> bool {
        let v = unsafe { spd_set_voice_rate(self.0, rate) };
        i32_to_bool(v)
    }

    pub fn set_voice_rate_all(&self, rate: i32) -> bool {
        let v = unsafe { spd_set_voice_rate_all(self.0, rate) };
        i32_to_bool(v)
    }

    pub fn set_voice_rate_uid(&self, rate: i32, target_uid: u32) -> bool {
        let v = unsafe { spd_set_voice_rate_uid(self.0, rate, target_uid) };
        i32_to_bool(v)
    }

    pub fn get_voice_rate(&self) -> i32 {
        unsafe { spd_get_voice_rate(self.0) }
    }

    pub fn set_voice_pitch(&self, pitch: i32) -> bool {
        let v = unsafe { spd_set_voice_pitch(self.0, pitch) };
        i32_to_bool(v)
    }

    pub fn set_voice_pitch_all(&self, pitch: i32) -> bool {
        let v = unsafe { spd_set_voice_pitch_all(self.0, pitch) };
        i32_to_bool(v)
    }

    pub fn set_voice_pitch_uid(&self, pitch: i32, target_uid: u32) -> bool {
        let v = unsafe { spd_set_voice_pitch_uid(self.0, pitch, target_uid) };
        i32_to_bool(v)
    }

    pub fn get_voice_pitch(&self) -> i32 {
        unsafe { spd_get_voice_pitch(self.0) }
    }

    pub fn set_volume(&self, volume: i32) -> bool {
        let v = unsafe { spd_set_volume(self.0, volume) };
        i32_to_bool(v)
    }

    pub fn set_volume_all(&self, volume: i32) -> bool {
        let v = unsafe { spd_set_volume_all(self.0, volume) };
        i32_to_bool(v)
    }

    pub fn set_volume_uid(&self, volume: i32, target_uid: u32) -> bool {
        let v = unsafe { spd_set_volume_uid(self.0, volume, target_uid) };
        i32_to_bool(v)
    }

    pub fn get_volume(&self) -> i32 {
        unsafe { spd_get_volume(self.0) }
    }

    pub fn set_punctuation(&self, punctuation: Punctuation) -> bool {
        let v = unsafe { spd_set_punctuation(self.0, punctuation as u32) };
        i32_to_bool(v)
    }

    pub fn set_punctuation_all(&self, punctuation: Punctuation) -> bool {
        let v = unsafe { spd_set_punctuation_all(self.0, punctuation as u32) };
        i32_to_bool(v)
    }

    pub fn set_punctuation_uid(&self, punctuation: Punctuation, target_uid: u32) -> bool {
        let v = unsafe { spd_set_punctuation_uid(self.0, punctuation as u32, target_uid) };
        i32_to_bool(v)
    }

    pub fn set_capital_letters(&self, capital_letters: CapitalLetters) -> bool {
        let v = unsafe { spd_set_capital_letters(self.0, capital_letters as u32) };
        i32_to_bool(v)
    }

    pub fn set_capital_letters_all(&self, capital_letters: CapitalLetters) -> bool {
        let v = unsafe { spd_set_capital_letters_all(self.0, capital_letters as u32) };
        i32_to_bool(v)
    }

    pub fn set_capital_letters_uid(
        &self,
        capital_letters: CapitalLetters,
        target_uid: u32,
    ) -> bool {
        let v = unsafe { spd_set_capital_letters_uid(self.0, capital_letters as u32, target_uid) };
        i32_to_bool(v)
    }

    pub fn set_spelling(&self, spelling: bool) -> bool {
        let s = if spelling {
            SPDSpelling::SPD_SPELL_ON
        } else {
            SPDSpelling::SPD_SPELL_OFF
        };
        let v = unsafe { spd_set_spelling(self.0, s) };
        i32_to_bool(v)
    }

    pub fn set_spelling_all(&self, spelling: bool) -> bool {
        let s = if spelling {
            SPDSpelling::SPD_SPELL_ON
        } else {
            SPDSpelling::SPD_SPELL_OFF
        };
        let v = unsafe { spd_set_spelling_all(self.0, s) };
        i32_to_bool(v)
    }

    pub fn set_spelling_uid(&self, spelling: bool, target_uid: u32) -> bool {
        let s = if spelling {
            SPDSpelling::SPD_SPELL_ON
        } else {
            SPDSpelling::SPD_SPELL_OFF
        };
        let v = unsafe { spd_set_spelling_uid(self.0, s, target_uid) };
        i32_to_bool(v)
    }

    pub fn set_language<S: Into<String>>(&self, language: S) -> bool {
        let param = CString::new(language.into()).unwrap();
        let v = unsafe { spd_set_language(self.0, param.as_ptr()) };
        i32_to_bool(v)
    }

    pub fn set_language_all<S: Into<String>>(&self, language: S) -> bool {
        let param = CString::new(language.into()).unwrap();
        let v = unsafe { spd_set_language_all(self.0, param.as_ptr()) };
        i32_to_bool(v)
    }

    pub fn set_language_uid<S: Into<String>>(&self, language: S, target_uid: u32) -> bool {
        let param = CString::new(language.into()).unwrap();
        let v = unsafe { spd_set_language_uid(self.0, param.as_ptr(), target_uid) };
        i32_to_bool(v)
    }

    pub fn get_language(&self) -> &str {
        let v = unsafe { CStr::from_ptr(spd_get_language(self.0)) };
        v.to_str().unwrap()
    }

    pub fn set_output_module<S: Into<String>>(&self, output_module: S) -> bool {
        let param = CString::new(output_module.into()).unwrap();
        let v = unsafe { spd_set_output_module(self.0, param.as_ptr()) };
        i32_to_bool(v)
    }

    pub fn set_output_module_all<S: Into<String>>(&self, output_module: S) -> bool {
        let param = CString::new(output_module.into()).unwrap();
        let v = unsafe { spd_set_output_module_all(self.0, param.as_ptr()) };
        i32_to_bool(v)
    }

    pub fn set_output_module_uid<S: Into<String>>(
        &self,
        output_module: S,
        target_uid: u32,
    ) -> bool {
        let param = CString::new(output_module.into()).unwrap();
        let v = unsafe { spd_set_output_module_uid(self.0, param.as_ptr(), target_uid) };
        i32_to_bool(v)
    }

    pub fn send_data<S: Into<String>>(&self, data: S, wait_for_reply: bool) -> Option<String> {
        let wfr: i32 = if wait_for_reply {
            SPD_WAIT_REPLY as i32
        } else {
            SPD_NO_REPLY as i32
        };
        let data = CString::new(data.into()).unwrap();
        let rv = unsafe { spd_send_data(self.0, data.as_ptr(), wfr) };
        if rv.is_null() {
            None
        } else {
            let rv = unsafe { CStr::from_ptr(rv) };
            Some(rv.to_string_lossy().to_string())
        }
    }

    pub fn on_begin(&self, f: Option<Box<dyn FnMut(u64, u64)>>) {
        if let Ok(mut cbs) = callbacks.lock() {
            let cb = cbs.get_mut(&self.1);
            if let Some(cb) = cb {
                cb.begin = f;
            }
        }
    }

    pub fn on_end(&self, f: Option<Box<dyn FnMut(u64, u64)>>) {
        if let Ok(mut cbs) = callbacks.lock() {
            let cb = cbs.get_mut(&self.1);
            if let Some(cb) = cb {
                cb.end = f;
            }
        }
    }

    pub fn on_cancel(&self, f: Option<Box<dyn FnMut(u64, u64)>>) {
        if let Ok(mut cbs) = callbacks.lock() {
            let cb = cbs.get_mut(&self.1);
            if let Some(cb) = cb {
                cb.cancel = f;
            }
        }
    }

    pub fn on_pause(&self, f: Option<Box<dyn FnMut(u64, u64)>>) {
        if let Ok(mut cbs) = callbacks.lock() {
            let cb = cbs.get_mut(&self.1);
            if let Some(cb) = cb {
                cb.pause = f;
            }
        }
    }

    pub fn on_resume(&self, f: Option<Box<dyn FnMut(u64, u64)>>) {
        if let Ok(mut cbs) = callbacks.lock() {
            let cb = cbs.get_mut(&self.1);
            if let Some(cb) = cb {
                cb.resume = f;
            }
        }
    }

    pub fn on_index_mark(&self, f: Option<Box<dyn FnMut(u64, u64, String)>>) {
        if let Ok(mut cbs) = callbacks.lock() {
            let cb = cbs.get_mut(&self.1);
            if let Some(cb) = cb {
                cb.index_mark = f;
            }
        }
    }

    pub fn client_id(&self) -> u64 {
        self.1
    }
}

unsafe impl Send for Connection {}

impl Drop for Connection {
    fn drop(&mut self) {
        self.close();
        callbacks.lock().unwrap().remove(&self.1);
    }
}
