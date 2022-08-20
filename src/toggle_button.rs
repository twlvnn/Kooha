use gtk::{glib, prelude::*, subclass::prelude::*};

use std::cell::{Cell, RefCell};

mod imp {
    use super::*;
    use once_cell::sync::Lazy;

    #[derive(Debug, Default)]
    pub struct ToggleButton {
        pub(super) is_active: Cell<bool>,
        pub(super) default_icon_name: RefCell<String>,
        pub(super) toggled_icon_name: RefCell<String>,
        pub(super) default_tooltip_text: RefCell<String>,
        pub(super) toggled_tooltip_text: RefCell<String>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ToggleButton {
        const NAME: &'static str = "MsaiToggleButton";
        type Type = super::ToggleButton;
        type ParentType = gtk::Button;
    }

    impl ObjectImpl for ToggleButton {
        fn properties() -> &'static [glib::ParamSpec] {
            static PROPERTIES: Lazy<Vec<glib::ParamSpec>> = Lazy::new(|| {
                vec![
                    // Icon name to show on un-toggled state
                    glib::ParamSpecBoolean::builder("active")
                        .flags(glib::ParamFlags::READWRITE | glib::ParamFlags::EXPLICIT_NOTIFY)
                        .build(),
                    // Icon name to show on un-toggled state
                    glib::ParamSpecString::builder("default-icon-name")
                        .flags(glib::ParamFlags::READWRITE | glib::ParamFlags::EXPLICIT_NOTIFY)
                        .build(),
                    // Icon name to show on toggled state
                    glib::ParamSpecString::builder("toggled-icon-name")
                        .flags(glib::ParamFlags::READWRITE | glib::ParamFlags::EXPLICIT_NOTIFY)
                        .build(),
                    // Icon name to show on un-toggled state
                    glib::ParamSpecString::builder("default-tooltip-text")
                        .flags(glib::ParamFlags::READWRITE | glib::ParamFlags::EXPLICIT_NOTIFY)
                        .build(),
                    // Icon name to show on toggled state
                    glib::ParamSpecString::builder("toggled-tooltip-text")
                        .flags(glib::ParamFlags::READWRITE | glib::ParamFlags::EXPLICIT_NOTIFY)
                        .build(),
                ]
            });

            PROPERTIES.as_ref()
        }

        fn set_property(
            &self,
            obj: &Self::Type,
            _id: usize,
            value: &glib::Value,
            pspec: &glib::ParamSpec,
        ) {
            match pspec.name() {
                "active" => {
                    let is_active = value.get().unwrap();
                    obj.set_active(is_active);
                }
                "default-icon-name" => {
                    let default_icon_name = value.get().unwrap();
                    obj.set_default_icon_name(default_icon_name);
                }
                "toggled-icon-name" => {
                    let toggled_icon_name = value.get().unwrap();
                    obj.set_toggled_icon_name(toggled_icon_name);
                }
                "default-tooltip-text" => {
                    let default_tooltip_text = value.get().unwrap();
                    obj.set_default_tooltip_text(default_tooltip_text);
                }
                "toggled-tooltip-text" => {
                    let toggled_tooltip_text = value.get().unwrap();
                    obj.set_toggled_tooltip_text(toggled_tooltip_text);
                }
                _ => unimplemented!(),
            }
        }

        fn property(&self, obj: &Self::Type, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
            match pspec.name() {
                "active" => obj.is_active().to_value(),
                "default-icon-name" => obj.default_icon_name().to_value(),
                "toggled-icon-name" => obj.toggled_icon_name().to_value(),
                "default-tooltip-text" => obj.default_tooltip_text().to_value(),
                "toggled-tooltip-text" => obj.toggled_tooltip_text().to_value(),
                _ => unimplemented!(),
            }
        }
    }

    impl WidgetImpl for ToggleButton {}

    impl ButtonImpl for ToggleButton {
        fn clicked(&self, obj: &Self::Type) {
            self.parent_clicked(obj);
            obj.set_active(!obj.is_active());
        }
    }
}

glib::wrapper! {
     pub struct ToggleButton(ObjectSubclass<imp::ToggleButton>)
        @extends gtk::Widget, gtk::Button;
}

impl ToggleButton {
    pub fn new() -> Self {
        glib::Object::new(&[]).expect("Failed to create MsaiToggleButton.")
    }

    pub fn set_active(&self, is_active: bool) {
        if is_active == self.is_active() {
            return;
        }

        self.imp().is_active.set(is_active);
        self.update_icon_name();
        self.update_tooltip_text();
        self.notify("active");
    }

    pub fn is_active(&self) -> bool {
        self.imp().is_active.get()
    }

    pub fn set_default_icon_name(&self, default_icon_name: &str) {
        if default_icon_name == self.default_icon_name().as_str() {
            return;
        }

        self.imp()
            .default_icon_name
            .replace(default_icon_name.to_string());
        self.update_icon_name();
        self.notify("default-icon-name");
    }

    pub fn default_icon_name(&self) -> String {
        self.imp().default_icon_name.borrow().clone()
    }

    pub fn set_toggled_icon_name(&self, toggled_icon_name: &str) {
        if toggled_icon_name == self.toggled_icon_name().as_str() {
            return;
        }

        self.imp()
            .toggled_icon_name
            .replace(toggled_icon_name.to_string());
        self.update_icon_name();
        self.notify("toggled-icon-name");
    }

    pub fn toggled_icon_name(&self) -> String {
        self.imp().toggled_icon_name.borrow().clone()
    }

    pub fn set_default_tooltip_text(&self, default_tooltip_text: &str) {
        if default_tooltip_text == self.default_tooltip_text().as_str() {
            return;
        }

        self.imp()
            .default_tooltip_text
            .replace(default_tooltip_text.to_string());
        self.update_tooltip_text();
        self.notify("default-tooltip-text");
    }

    pub fn default_tooltip_text(&self) -> String {
        self.imp().default_tooltip_text.borrow().clone()
    }

    pub fn set_toggled_tooltip_text(&self, toggled_tooltip_text: &str) {
        if toggled_tooltip_text == self.toggled_tooltip_text().as_str() {
            return;
        }

        self.imp()
            .toggled_tooltip_text
            .replace(toggled_tooltip_text.to_string());
        self.update_tooltip_text();
        self.notify("toggled-tooltip-text");
    }

    pub fn toggled_tooltip_text(&self) -> String {
        self.imp().toggled_tooltip_text.borrow().clone()
    }

    fn update_icon_name(&self) {
        let icon_name = if self.is_active() {
            self.toggled_icon_name()
        } else {
            self.default_icon_name()
        };
        self.set_icon_name(&icon_name);
    }

    fn update_tooltip_text(&self) {
        let tooltip_text = if self.is_active() {
            self.toggled_tooltip_text()
        } else {
            self.default_tooltip_text()
        };
        self.set_tooltip_text(if tooltip_text.is_empty() {
            None
        } else {
            Some(&tooltip_text)
        });
    }
}

impl Default for ToggleButton {
    fn default() -> Self {
        Self::new()
    }
}
