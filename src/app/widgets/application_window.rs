// Copyright Sebastian Wiesner <sebastian@swsnr.de>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use adw::prelude::*;
use gtk::gio;

glib::wrapper! {
    pub struct UmbrellaApplicationWindow(ObjectSubclass<imp::UmbrellaApplicationWindow>)
        @extends adw::ApplicationWindow, gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap,
            gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget,
            gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl UmbrellaApplicationWindow {
    /// Create a new application window for the given `application`.
    pub fn new(application: &impl IsA<gtk::Application>, startpage_icon_name: &str) -> Self {
        glib::Object::builder()
            .property("application", application)
            .property("startpage-icon-name", startpage_icon_name)
            .build()
    }
}

mod imp {
    use std::cell::RefCell;

    use adw::prelude::*;
    use adw::subclass::prelude::*;
    use glib::subclass::InitializingObject;
    use gtk::CompositeTemplate;

    #[derive(Default, CompositeTemplate, glib::Properties)]
    #[properties(wrapper_type = super::UmbrellaApplicationWindow)]
    #[template(resource = "/de/swsnr/umbrella/ui/application-window.ui")]
    pub struct UmbrellaApplicationWindow {
        #[property(get, set)]
        startpage_icon_name: RefCell<String>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for UmbrellaApplicationWindow {
        const NAME: &'static str = "UmbrellaApplicationWindow";

        type Type = super::UmbrellaApplicationWindow;

        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for UmbrellaApplicationWindow {}

    impl WidgetImpl for UmbrellaApplicationWindow {}

    impl WindowImpl for UmbrellaApplicationWindow {}

    impl ApplicationWindowImpl for UmbrellaApplicationWindow {}

    impl AdwApplicationWindowImpl for UmbrellaApplicationWindow {}
}
