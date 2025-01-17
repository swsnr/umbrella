// Copyright Sebastian Wiesner <sebastian@swsnr.de>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use glib::Object;

glib::wrapper! {
    pub struct UmbrellaPreferencesDialog(ObjectSubclass<imp::UmbrellaPreferencesDialog>)
        @extends adw::PreferencesDialog, adw::Dialog, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl Default for UmbrellaPreferencesDialog {
    fn default() -> Self {
        Object::builder().build()
    }
}

mod imp {
    use std::cell::RefCell;

    use adw::prelude::*;
    use adw::subclass::prelude::*;
    use glib::subclass::InitializingObject;
    use gtk::CompositeTemplate;

    #[derive(Default, Debug, CompositeTemplate, glib::Properties)]
    #[properties(wrapper_type = super::UmbrellaPreferencesDialog)]
    #[template(resource = "/de/swsnr/umbrella/ui/preferences-dialog.ui")]
    pub struct UmbrellaPreferencesDialog {
        // TODO: Warn when not entering an SFTP repository
        #[property(get, set)]
        repository_uri: RefCell<String>,
        #[property(get, set)]
        repository_key: RefCell<String>,
        #[property(get, set)]
        sftp_password: RefCell<String>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for UmbrellaPreferencesDialog {
        const NAME: &'static str = "UmbrellaPreferencesDialog";

        type Type = super::UmbrellaPreferencesDialog;

        type ParentType = adw::PreferencesDialog;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for UmbrellaPreferencesDialog {}

    impl WidgetImpl for UmbrellaPreferencesDialog {}

    impl AdwDialogImpl for UmbrellaPreferencesDialog {}

    impl PreferencesDialogImpl for UmbrellaPreferencesDialog {}
}
