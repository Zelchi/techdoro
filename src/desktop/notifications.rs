use std::time::Duration;

pub fn notify_finished(focus: bool) {
    std::thread::spawn(move || {
        let mut notification = notify_rust::Notification::new();
        #[cfg(target_os = "linux")]
        notification
            .icon("techdoro")
            .hint(notify_rust::Hint::DesktopEntry(
                "com.zelchi.techdoro".into(),
            ));
        #[cfg(target_os = "windows")]
        notification.app_id("com.zelchi.techdoro");
        if let Err(error) = notification
            .appname("Techdoro")
            .summary(if focus {
                "Focus session complete!"
            } else {
                "Break complete!"
            })
            .body(if focus {
                "Time to stretch your legs!"
            } else {
                "Time to get back to work."
            })
            .timeout(Duration::from_secs(8))
            .show()
        {
            eprintln!("Could not send notification: {error}");
        }
    });
}
