FROM ghcr.io/gtk-rs/gtk4-rs/gtk4:latest

RUN git clone https://gitlab.gnome.org/GNOME/libsecret.git --depth=1 && \
    (cd /libsecret && \
        meson setup builddir --prefix=/usr --buildtype release -Dmanpage=false -Dvapi=false -Dgtk_doc=false -Dintrospection=false -Dbash_completion=disabled && \
        meson install -C builddir) && \
    rm -rf /libsecret
