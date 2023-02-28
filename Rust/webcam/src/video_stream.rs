use tracing::info;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{HtmlVideoElement, MediaStream, MediaStreamConstraints};

pub struct VideoStream {
    el: HtmlVideoElement,
}

impl VideoStream {
    pub fn new(el: HtmlVideoElement) -> VideoStream {
        VideoStream { el }
    }

    pub async fn set_video_src(&self, constraints_serde_json: &serde_json::Value) {
        let window = web_sys::window().expect("window获取失败！");
        let navigator = window.navigator();
        let media_devices = navigator.media_devices().expect("mediaDevices get error");
        info!("devices (tracing_wasm): {:?}", media_devices);
        web_sys::console::log_1(&media_devices);
        // mediaDevices.get_user_media(constraints);
        let mut constraints = MediaStreamConstraints::new();
        constraints.video(
            &serde_wasm_bindgen::to_value(constraints_serde_json)
                .expect("serde_json::Value to JsValue Error"),
        );
        constraints.audio(&false.into());

        let with_constraints = media_devices
            .get_user_media_with_constraints(&constraints)
            .unwrap();
        web_sys::console::log_1(&with_constraints);
        let media_jsv = JsFuture::from(with_constraints).await.unwrap();
        web_sys::console::log_1(&"12345514432123".into());
        let media_stream = media_jsv.unchecked_into::<MediaStream>();
        // let media_stream = MediaStream::new_with_tracks(&media_jsv);
        info!("media_stream - (tracing_wasm) {:?}", media_stream);
        self.el.set_src_object(Some(&media_stream));
    }
}
