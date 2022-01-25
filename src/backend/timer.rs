use gtk::{
    glib::{self, clone},
    prelude::*,
    subclass::prelude::*,
};
use once_cell::sync::Lazy;

use std::cell::Cell;

#[derive(Debug, PartialEq, Clone, Copy, glib::Enum)]
#[enum_type(name = "KoohaTimerState")]
pub enum TimerState {
    Stopped,
    Delayed,
    Paused,
    Running,
}

impl Default for TimerState {
    fn default() -> Self {
        Self::Stopped
    }
}

mod imp {
    use super::*;
    use glib::subclass::Signal;

    #[derive(Debug, Default)]
    pub struct Timer {
        pub state: Cell<TimerState>,
        pub time: Cell<u32>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Timer {
        const NAME: &'static str = "KoohaTimer";
        type Type = super::Timer;
    }

    impl ObjectImpl for Timer {
        fn signals() -> &'static [Signal] {
            static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
                vec![Signal::builder("delay-done", &[], <()>::static_type().into()).build()]
            });
            SIGNALS.as_ref()
        }

        fn properties() -> &'static [glib::ParamSpec] {
            static PROPERTIES: Lazy<Vec<glib::ParamSpec>> = Lazy::new(|| {
                vec![
                    glib::ParamSpecEnum::new(
                        "state",
                        "state",
                        "Current state of Self",
                        TimerState::static_type(),
                        TimerState::default() as i32,
                        glib::ParamFlags::READWRITE,
                    ),
                    glib::ParamSpecUInt::new(
                        "time",
                        "time",
                        "Current time",
                        0,
                        std::u32::MAX as u32,
                        0,
                        glib::ParamFlags::READWRITE,
                    ),
                ]
            });
            PROPERTIES.as_ref()
        }

        fn set_property(
            &self,
            _obj: &Self::Type,
            _id: usize,
            value: &glib::Value,
            pspec: &glib::ParamSpec,
        ) {
            match pspec.name() {
                "state" => {
                    let state = value.get().unwrap();
                    self.state.set(state);
                }
                "time" => {
                    let time = value.get().unwrap();
                    self.time.set(time);
                }
                _ => unimplemented!(),
            }
        }

        fn property(&self, _obj: &Self::Type, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
            match pspec.name() {
                "state" => self.state.get().to_value(),
                "time" => self.time.get().to_value(),
                _ => unimplemented!(),
            }
        }
    }
}

glib::wrapper! {
    pub struct Timer(ObjectSubclass<imp::Timer>);
}

impl Timer {
    pub fn new() -> Self {
        glib::Object::new::<Self>(&[]).expect("Failed to create Timer.")
    }

    fn set_state(&self, new_state: TimerState) {
        self.set_property("state", new_state);
    }

    pub fn state(&self) -> TimerState {
        self.property("state")
    }

    fn set_time(&self, new_time: u32) {
        self.set_property("time", new_time);
    }

    fn time(&self) -> u32 {
        self.property("time")
    }

    fn update_time(&self) {
        let current_time = self.time();

        let new_time = if self.state() == TimerState::Delayed {
            current_time - 1
        } else {
            current_time + 1
        };

        self.set_time(new_time);
    }

    pub fn connect_state_notify<F>(&self, f: F) -> glib::SignalHandlerId
    where
        F: Fn(&Self) + 'static,
    {
        self.connect_notify_local(Some("state"), move |obj, _| f(obj))
    }

    pub fn connect_time_notify<F>(&self, f: F) -> glib::SignalHandlerId
    where
        F: Fn(&Self) + 'static,
    {
        self.connect_notify_local(Some("time"), move |obj, _| f(obj))
    }

    pub fn connect_delay_done<F>(&self, f: F) -> glib::SignalHandlerId
    where
        F: Fn(&Self) + 'static,
    {
        self.connect_local("delay-done", true, move |values| {
            let obj = values[0].get::<Self>().unwrap();
            f(&obj);
            None
        })
    }

    pub fn start(&self, delay: u32) {
        self.set_time(delay);

        glib::timeout_add_seconds_local(
            1,
            clone!(@weak self as obj => @default-return Continue(false), move || {
                let current_state = obj.state();

                if current_state == TimerState::Stopped {
                    return Continue(false);
                }

                if current_state != TimerState::Paused {
                    obj.update_time();
                }

                if obj.time() == 0 && current_state == TimerState::Delayed {
                    obj.set_state(TimerState::Running);
                    obj.emit_by_name::<()>("delay-done", &[]);
                }

                Continue(true)
            }),
        );

        if delay == 0 {
            self.set_state(TimerState::Running);
            self.emit_by_name::<()>("delay-done", &[]);
        } else {
            self.set_state(TimerState::Delayed);
        }
    }

    pub fn pause(&self) {
        self.set_state(TimerState::Paused);
    }

    pub fn resume(&self) {
        self.set_state(TimerState::Running);
    }

    pub fn stop(&self) {
        self.set_state(TimerState::Stopped);
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self::new()
    }
}
