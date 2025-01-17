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
    use adw::subclass::prelude::*;
    use glib::subclass::InitializingObject;
    use gtk::CompositeTemplate;

    #[derive(Default, CompositeTemplate)]
    #[template(resource = "/de/swsnr/umbrella/ui/preferences-dialog.ui")]
    pub struct UmbrellaPreferencesDialog {}

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

    impl ObjectImpl for UmbrellaPreferencesDialog {}

    impl WidgetImpl for UmbrellaPreferencesDialog {}

    impl AdwDialogImpl for UmbrellaPreferencesDialog {}

    impl PreferencesDialogImpl for UmbrellaPreferencesDialog {}
}
