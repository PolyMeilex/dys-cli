use glib::translate::ToGlibPtr;

pub trait GPrint {
    fn print<P: AsRef<std::ffi::OsStr>>(&self, format: P);

    fn print_err<P: AsRef<std::ffi::OsStr>>(&self, format: P);
}

impl GPrint for gio::ApplicationCommandLine {
    fn print<P: AsRef<std::ffi::OsStr>>(&self, format: P) {
        unsafe {
            gio_sys::g_application_command_line_print(
                self.to_glib_none().0,
                format.as_ref().to_glib_none().0,
            );
        }
    }

    fn print_err<P: AsRef<std::ffi::OsStr>>(&self, format: P) {
        unsafe {
            gio_sys::g_application_command_line_printerr(
                self.to_glib_none().0,
                format.as_ref().to_glib_none().0,
            );
        }
    }
}
