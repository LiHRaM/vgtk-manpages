pub(crate) use webkit2gtk::{WebView, WebViewExt};

pub(crate) trait WebViewHelpers: WebViewExt {
    fn get_html(&self) -> String {
        String::from("No idea how to get the html, sorry")
    }

    fn set_html(&self, html: String) {
        self.load_html(&html, None);
    }
}

impl<WV> WebViewHelpers for WV where WV: WebViewExt {}


