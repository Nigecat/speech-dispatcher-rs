#![allow(non_upper_case_globals)]
extern crate speech_dispatcher_sys;

use std::ffi::{CStr, CString};
use std::marker::Send;

use speech_dispatcher_sys::*;

pub enum Mode {
    Single = SPDConnectionMode::SPD_MODE_SINGLE as isize,
    Threaded = SPDConnectionMode::SPD_MODE_THREADED as isize,
}

pub enum Priority {
    Important = SPDPriority::SPD_IMPORTANT as isize,
    Message = SPDPriority::SPD_MESSAGE as isize,
    Text = SPDPriority::SPD_TEXT as isize,
    Notification = SPDPriority::SPD_NOTIFICATION as isize,
    Progress = SPDPriority::SPD_PROGRESS as isize,
}

pub enum VoiceType {
    Male1 = SPDVoiceType::SPD_MALE1 as isize,
    Male2 = SPDVoiceType::SPD_MALE2 as isize,
    Male3 = SPDVoiceType::SPD_MALE3 as isize,
    Female1 = SPDVoiceType::SPD_FEMALE1 as isize,
    Female2 = SPDVoiceType::SPD_FEMALE2 as isize,
    Female3 = SPDVoiceType::SPD_FEMALE3 as isize,
    ChildMale = SPDVoiceType::SPD_CHILD_MALE as isize,
    ChildFemale = SPDVoiceType::SPD_CHILD_FEMALE as isize,
}

pub struct Connection {
    connection: *mut SPDConnection,
}

pub type Address = SPDConnectionAddress;

pub enum DataMode {
    Text = SPDDataMode::SPD_DATA_TEXT as isize,
    SSML = SPDDataMode::SPD_DATA_SSML as isize,
}

pub enum Notification {
    Begin = SPDNotification::SPD_BEGIN as isize,
    End = SPDNotification::SPD_END as isize,
    IndexMarks = SPDNotification::SPD_INDEX_MARKS as isize,
    Cancel = SPDNotification::SPD_CANCEL as isize,
    Pause = SPDNotification::SPD_PAUSE as isize,
    Resume = SPDNotification::SPD_RESUME as isize,
    All = SPDNotification::SPD_ALL as isize,
}

pub enum Punctuation {
    All = SPDPunctuation::SPD_PUNCT_ALL as isize,
    None = SPDPunctuation::SPD_PUNCT_NONE as isize,
    Some = SPDPunctuation::SPD_PUNCT_SOME as isize,
}

pub enum CapitalLetters {
    None = SPDCapitalLetters::SPD_CAP_NONE as isize,
    Spell = SPDCapitalLetters::SPD_CAP_SPELL as isize,
    Icon = SPDCapitalLetters::SPD_CAP_ICON as isize,
}

fn i32_to_bool(v: i32) -> bool {
    v == 1
}

impl Connection {
    pub fn open<S: Into<String>>(
        client_name: S,
        connection_name: S,
        user_name: S,
        mode: Mode,
    ) -> Connection {
        let clientname = CString::new(client_name.into()).unwrap();
        let connectionname = CString::new(connection_name.into()).unwrap();
        let username = CString::new(user_name.into()).unwrap();
        let connection = unsafe {
            spd_open(
                clientname.as_ptr(),
                connectionname.as_ptr(),
                username.as_ptr(),
                mode as u32,
            )
        };
        Connection { connection }
    }

    pub unsafe fn open2<S: Into<String>>(
        client_name: S,
        connection_name: S,
        user_name: S,
        mode: Mode,
        address: *mut Address,
        autospawn: bool,
    ) -> Connection {
        let auto_spawn = if autospawn { 1 } else { 0 };
        let error_result = vec![CString::new("").unwrap().into_raw()].as_mut_ptr();
        let clientname = CString::new(client_name.into()).unwrap();
        let connectionname = CString::new(connection_name.into()).unwrap();
        let username = CString::new(user_name.into()).unwrap();
        let connection = spd_open2(
            clientname.as_ptr(),
            connectionname.as_ptr(),
            username.as_ptr(),
            mode as u32,
            address,
            auto_spawn,
            error_result,
        );
        Connection { connection }
    }

    pub fn close(&self) {
        unsafe { spd_close(self.connection) };
    }

    pub fn say<S: Into<String>>(&self, priority: Priority, text: S) -> bool {
        let text: String = text.into();
        if text.is_empty() {
            return true;
        }
        let param = CString::new(text).unwrap();
        let v = unsafe { spd_say(self.connection, priority as u32, param.as_ptr()) };
        i32_to_bool(v)
    }

    pub fn sayf<S: Into<String>>(&self, priority: Priority, format: S) -> bool {
        let format: String = format.into();
        if format.is_empty() {
            return true;
        }
        let param = CString::new(format).unwrap();
        let v = unsafe { spd_sayf(self.connection, priority as u32, param.as_ptr()) };
        i32_to_bool(v)
    }

    pub fn stop(&self) -> bool {
        let v = unsafe { spd_stop(self.connection) };
        i32_to_bool(v)
    }

    pub fn stop_all(&self) -> bool {
        let v = unsafe { spd_stop_all(self.connection) };
        i32_to_bool(v)
    }

    pub fn stop_uid(&self, target_uid: i32) -> bool {
        let v = unsafe { spd_stop_uid(self.connection, target_uid) };
        i32_to_bool(v)
    }

    pub fn cancel(&self) -> bool {
        let v = unsafe { spd_cancel(self.connection) };
        i32_to_bool(v)
    }

    pub fn cancel_all(&self) -> bool {
        let v = unsafe { spd_cancel_all(self.connection) };
        i32_to_bool(v)
    }

    pub fn cancel_uid(&self, target_uid: i32) -> bool {
        let v = unsafe { spd_cancel_uid(self.connection, target_uid) };
        i32_to_bool(v)
    }

    pub fn pause(&self) -> bool {
        let v = unsafe { spd_pause(self.connection) };
        i32_to_bool(v)
    }

    pub fn pause_all(&self) -> bool {
        let v = unsafe { spd_pause_all(self.connection) };
        i32_to_bool(v)
    }

    pub fn pause_uid(&self, target_uid: i32) -> bool {
        let v = unsafe { spd_pause_uid(self.connection, target_uid) };
        i32_to_bool(v)
    }

    pub fn resume(&self) -> bool {
        let v = unsafe { spd_resume(self.connection) };
        i32_to_bool(v)
    }

    pub fn resume_all(&self) -> bool {
        let v = unsafe { spd_resume_all(self.connection) };
        i32_to_bool(v)
    }

    pub fn resume_uid(&self, target_uid: i32) -> bool {
        let v = unsafe { spd_resume_uid(self.connection, target_uid) };
        i32_to_bool(v)
    }

    pub fn key<S: Into<String>>(&self, priority: Priority, key_name: S) -> bool {
        let param = CString::new(key_name.into()).unwrap();
        let v = unsafe { spd_key(self.connection, priority as u32, param.as_ptr()) };
        i32_to_bool(v)
    }

    pub fn char<S: Into<String>>(&self, priority: Priority, char: S) -> bool {
        let param = CString::new(char.into()).unwrap();
        let v = unsafe { spd_char(self.connection, priority as u32, param.as_ptr()) };
        i32_to_bool(v)
    }

    pub fn wchar(&self, priority: Priority, wchar: i32) -> bool {
        let v = unsafe { spd_wchar(self.connection, priority as u32, wchar) };
        i32_to_bool(v)
    }

    pub fn sound_icon<S: Into<String>>(&self, priority: Priority, icon_name: S) -> bool {
        let param = CString::new(icon_name.into()).unwrap();
        let v = unsafe { spd_char(self.connection, priority as u32, param.as_ptr()) };
        i32_to_bool(v)
    }

    pub fn set_voice_type(&self, voice_type: VoiceType) -> bool {
        let v = unsafe { spd_set_voice_type(self.connection, voice_type as u32) };
        i32_to_bool(v)
    }

    pub fn set_voice_type_all(&self, voice_type: VoiceType) -> bool {
        let v = unsafe { spd_set_voice_type_all(self.connection, voice_type as u32) };
        i32_to_bool(v)
    }

    pub fn set_voice_type_uid(&self, voice_type: VoiceType, target_uid: u32) -> bool {
        let v = unsafe { spd_set_voice_type_uid(self.connection, voice_type as u32, target_uid) };
        i32_to_bool(v)
    }

    pub fn get_voice_type(&self) -> VoiceType {
        let v = unsafe { spd_get_voice_type(self.connection) };
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
        let v = unsafe { spd_set_synthesis_voice(self.connection, param.as_ptr()) };
        i32_to_bool(v)
    }

    pub fn set_synthesis_voice_all<S: Into<String>>(&self, voice_name: S) -> bool {
        let param = CString::new(voice_name.into()).unwrap();
        let v = unsafe { spd_set_synthesis_voice_all(self.connection, param.as_ptr()) };
        i32_to_bool(v)
    }

    pub fn set_synthesis_voice_uid<S: Into<String>>(&self, voice_name: S, target_uid: u32) -> bool {
        let param = CString::new(voice_name.into()).unwrap();
        let v = unsafe { spd_set_synthesis_voice_uid(self.connection, param.as_ptr(), target_uid) };
        i32_to_bool(v)
    }

    pub fn set_data_mode(&self, mode: DataMode) -> bool {
        let v = unsafe { spd_set_data_mode(self.connection, mode as u32) };
        i32_to_bool(v)
    }

    pub fn set_notification_on(&self, notification: Notification) -> bool {
        let v = unsafe { spd_set_notification_on(self.connection, notification as u32) };
        i32_to_bool(v)
    }

    pub fn set_notification_off(&self, notification: Notification) -> bool {
        let v = unsafe { spd_set_notification_off(self.connection, notification as u32) };
        i32_to_bool(v)
    }

    pub fn set_notification<S: Into<String>>(&self, notification: Notification, state: S) -> bool {
        let param = CString::new(state.into()).unwrap();
        let v =
            unsafe { spd_set_notification(self.connection, notification as u32, param.as_ptr()) };
        i32_to_bool(v)
    }

    pub fn set_voice_rate(&self, rate: i32) -> bool {
        let v = unsafe { spd_set_voice_rate(self.connection, rate) };
        i32_to_bool(v)
    }

    pub fn set_voice_rate_all(&self, rate: i32) -> bool {
        let v = unsafe { spd_set_voice_rate_all(self.connection, rate) };
        i32_to_bool(v)
    }

    pub fn set_voice_rate_uid(&self, rate: i32, target_uid: u32) -> bool {
        let v = unsafe { spd_set_voice_rate_uid(self.connection, rate, target_uid) };
        i32_to_bool(v)
    }

    pub fn get_voice_rate(&self) -> i32 {
        unsafe { spd_get_voice_rate(self.connection) }
    }

    pub fn set_voice_pitch(&self, pitch: i32) -> bool {
        let v = unsafe { spd_set_voice_pitch(self.connection, pitch) };
        i32_to_bool(v)
    }

    pub fn set_voice_pitch_all(&self, pitch: i32) -> bool {
        let v = unsafe { spd_set_voice_pitch_all(self.connection, pitch) };
        i32_to_bool(v)
    }

    pub fn set_voice_pitch_uid(&self, pitch: i32, target_uid: u32) -> bool {
        let v = unsafe { spd_set_voice_pitch_uid(self.connection, pitch, target_uid) };
        i32_to_bool(v)
    }

    pub fn get_voice_pitch(&self) -> i32 {
        unsafe { spd_get_voice_pitch(self.connection) }
    }

    pub fn set_volume(&self, volume: i32) -> bool {
        let v = unsafe { spd_set_volume(self.connection, volume) };
        i32_to_bool(v)
    }

    pub fn set_volume_all(&self, volume: i32) -> bool {
        let v = unsafe { spd_set_volume_all(self.connection, volume) };
        i32_to_bool(v)
    }

    pub fn set_volume_uid(&self, volume: i32, target_uid: u32) -> bool {
        let v = unsafe { spd_set_volume_uid(self.connection, volume, target_uid) };
        i32_to_bool(v)
    }

    pub fn get_volume(&self) -> i32 {
        unsafe { spd_get_volume(self.connection) }
    }

    pub fn set_punctuation(&self, punctuation: Punctuation) -> bool {
        let v = unsafe { spd_set_punctuation(self.connection, punctuation as u32) };
        i32_to_bool(v)
    }

    pub fn set_punctuation_all(&self, punctuation: Punctuation) -> bool {
        let v = unsafe { spd_set_punctuation_all(self.connection, punctuation as u32) };
        i32_to_bool(v)
    }

    pub fn set_punctuation_uid(&self, punctuation: Punctuation, target_uid: u32) -> bool {
        let v = unsafe { spd_set_punctuation_uid(self.connection, punctuation as u32, target_uid) };
        i32_to_bool(v)
    }

    pub fn set_capital_letters(&self, capital_letters: CapitalLetters) -> bool {
        let v = unsafe { spd_set_capital_letters(self.connection, capital_letters as u32) };
        i32_to_bool(v)
    }

    pub fn set_capital_letters_all(&self, capital_letters: CapitalLetters) -> bool {
        let v = unsafe { spd_set_capital_letters_all(self.connection, capital_letters as u32) };
        i32_to_bool(v)
    }

    pub fn set_capital_letters_uid(
        &self,
        capital_letters: CapitalLetters,
        target_uid: u32,
    ) -> bool {
        let v = unsafe {
            spd_set_capital_letters_uid(self.connection, capital_letters as u32, target_uid)
        };
        i32_to_bool(v)
    }

    pub fn set_spelling(&self, spelling: bool) -> bool {
        let s = if spelling {
            SPDSpelling::SPD_SPELL_ON
        } else {
            SPDSpelling::SPD_SPELL_OFF
        };
        let v = unsafe { spd_set_spelling(self.connection, s) };
        i32_to_bool(v)
    }

    pub fn set_spelling_all(&self, spelling: bool) -> bool {
        let s = if spelling {
            SPDSpelling::SPD_SPELL_ON
        } else {
            SPDSpelling::SPD_SPELL_OFF
        };
        let v = unsafe { spd_set_spelling_all(self.connection, s) };
        i32_to_bool(v)
    }

    pub fn set_spelling_uid(&self, spelling: bool, target_uid: u32) -> bool {
        let s = if spelling {
            SPDSpelling::SPD_SPELL_ON
        } else {
            SPDSpelling::SPD_SPELL_OFF
        };
        let v = unsafe { spd_set_spelling_uid(self.connection, s, target_uid) };
        i32_to_bool(v)
    }

    pub fn set_language<S: Into<String>>(&self, language: S) -> bool {
        let param = CString::new(language.into()).unwrap();
        let v = unsafe { spd_set_language(self.connection, param.as_ptr()) };
        i32_to_bool(v)
    }

    pub fn set_language_all<S: Into<String>>(&self, language: S) -> bool {
        let param = CString::new(language.into()).unwrap();
        let v = unsafe { spd_set_language_all(self.connection, param.as_ptr()) };
        i32_to_bool(v)
    }

    pub fn set_language_uid<S: Into<String>>(&self, language: S, target_uid: u32) -> bool {
        let param = CString::new(language.into()).unwrap();
        let v = unsafe { spd_set_language_uid(self.connection, param.as_ptr(), target_uid) };
        i32_to_bool(v)
    }

    pub fn get_language(&self) -> &str {
        let v = unsafe { CStr::from_ptr(spd_get_language(self.connection)) };
        v.to_str().unwrap()
    }

    pub fn set_output_module<S: Into<String>>(&self, output_module: S) -> bool {
        let param = CString::new(output_module.into()).unwrap();
        let v = unsafe { spd_set_output_module(self.connection, param.as_ptr()) };
        i32_to_bool(v)
    }

    pub fn set_output_module_all<S: Into<String>>(&self, output_module: S) -> bool {
        let param = CString::new(output_module.into()).unwrap();
        let v = unsafe { spd_set_output_module_all(self.connection, param.as_ptr()) };
        i32_to_bool(v)
    }

    pub fn set_output_module_uid<S: Into<String>>(
        &self,
        output_module: S,
        target_uid: u32,
    ) -> bool {
        let param = CString::new(output_module.into()).unwrap();
        let v = unsafe { spd_set_output_module_uid(self.connection, param.as_ptr(), target_uid) };
        i32_to_bool(v)
    }
}

unsafe impl Send for Connection {}

impl Drop for Connection {
    fn drop(&mut self) {
        self.close();
    }
}
