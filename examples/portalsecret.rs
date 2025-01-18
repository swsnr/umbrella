// Copyright Sebastian Wiesner <sebastian@swsnr.de>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![deny(warnings, clippy::all, clippy::pedantic)]

use std::{
    cell::RefCell,
    fs::File,
    future::Future,
    io::Read,
    os::fd::{FromRawFd, OwnedFd},
    pin::Pin,
    rc::Rc,
};

use glib::{variant::Handle, VariantTy};
use gtk::{
    gio::{self, DBusConnection, IOErrorEnum},
    prelude::*,
};

fn create_pipe() -> Result<(OwnedFd, OwnedFd), glib::Error> {
    let (fd1, fd2) = glib::unix_open_pipe(libc::O_CLOEXEC)?;
    // SAFETY:: Both fds are ours at this point
    unsafe { Ok((OwnedFd::from_raw_fd(fd1), OwnedFd::from_raw_fd(fd2))) }
}

fn receive_portal_response(
    connection: &DBusConnection,
    request_path: &str,
) -> Pin<Box<dyn Future<Output = Result<glib::Variant, glib::Error>> + 'static>> {
    let request_path = request_path.to_owned();
    Box::pin(gio::GioFuture::new(
        connection,
        move |connection, cancellable, result| {
            type State = (
                gio::SignalSubscriptionId,
                gio::GioFutureResult<Result<glib::Variant, glib::Error>>,
            );
            let state: Rc<RefCell<Option<State>>> = Rc::new(RefCell::new(None));

            cancellable.connect_cancelled_local(glib::clone!(
                #[strong]
                connection,
                #[strong]
                state,
                move |_| {
                    if let Some((id, result)) = state.take() {
                        connection.signal_unsubscribe(id);
                        result.resolve(Err(glib::Error::new(IOErrorEnum::Cancelled, "cancelled")));
                    }
                }
            ));

            let id = connection.signal_subscribe(
                Some("org.freedesktop.portal.Desktop"),
                Some("org.freedesktop.portal.Request"),
                Some("Response"),
                Some(&request_path),
                None,
                gio::DBusSignalFlags::NO_MATCH_RULE,
                glib::clone!(
                    #[strong]
                    state,
                    move |connection, sender, path, interface, signal, parameters| {
                        dbg!(sender, path, interface, signal, parameters);
                        if let Some((id, result)) = state.take() {
                            connection.signal_unsubscribe(id);
                            result.resolve(Ok(parameters.clone()));
                        };
                    }
                ),
            );

            state.replace(Some((id, result)));
        },
    ))
}

fn activate(app: &gio::Application) {
    let connection = app.dbus_connection().unwrap();
    let token = "swsnrexample12335";
    let sender = connection
        .unique_name()
        .unwrap()
        .trim_start_matches(':')
        .replace('.', "_");
    let request_path = format!("/org/freedesktop/portal/desktop/request/{sender}/{token}");
    dbg!(&request_path);

    let fds = create_pipe().unwrap();
    let fdlist = gio::UnixFDList::new();
    let fdindex = fdlist.append(fds.1).unwrap();

    glib::spawn_future_local(glib::clone!(
        #[strong]
        app,
        #[strong]
        connection,
        async move {
            let output = receive_portal_response(&connection, &request_path)
                .await
                .unwrap();
            dbg!(output);
            let secret = gio::spawn_blocking(|| {
                let mut source: File = fds.0.into();
                let mut buf = Vec::with_capacity(1024);
                source.read_to_end(&mut buf)?;
                Ok::<Vec<u8>, std::io::Error>(buf)
            })
            .await
            .unwrap()
            .unwrap();
            dbg!(secret);
            app.quit();
        }
    ));

    glib::spawn_future_local(async move {
        let options = glib::VariantDict::new(None);
        options.insert("handle_token", token);
        let (result, result_fds) = connection
            .call_with_unix_fd_list_future(
                Some("org.freedesktop.portal.Desktop"),
                "/org/freedesktop/portal/desktop",
                "org.freedesktop.portal.Secret",
                "RetrieveSecret",
                Some(&((Handle(fdindex), options).into())),
                Some(VariantTy::new("(o)").unwrap()),
                gio::DBusCallFlags::NONE,
                -1,
                Some(&fdlist),
            )
            .await
            .unwrap();
        assert!(result_fds.is_none());
        dbg!(result);
    });
}

fn main() -> glib::ExitCode {
    static GLIB_LOGGER: glib::GlibLogger = glib::GlibLogger::new(
        glib::GlibLoggerFormat::Structured,
        glib::GlibLoggerDomain::CrateTarget,
    );
    let max_level = if std::env::var_os("G_MESSAGES_DEBUG").is_some() {
        log::LevelFilter::Trace
    } else {
        log::LevelFilter::Warn
    };
    log::set_max_level(max_level);
    log::set_logger(&GLIB_LOGGER).unwrap();

    let app = gio::Application::new(
        Some("de.swsnr.example.portalsecret"),
        gio::ApplicationFlags::FLAGS_NONE,
    );
    let _guard = app.hold();
    app.connect_activate(activate);
    app.run()
}
