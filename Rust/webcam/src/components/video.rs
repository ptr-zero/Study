use serde_json::json;
use sycamore::{futures, noderef, prelude::*};

use crate::VideoStream;

#[component]
pub fn Video<G: Html>(ctx: BoundedScope) -> View<G> {
    let video_ref = noderef::create_node_ref(ctx);
    sycamore::web::on_mount(ctx, move || {
        let el = video_ref.get::<DomNode>().unchecked_into();
        futures::spawn_local_scoped(ctx, async move {
            let video_stream = VideoStream::new(el);
            video_stream
                .set_video_src(&json!({
                    "audio":false,
                    "video":{ "facingMode": "environment", "width": 640, "height": 480 },
                }))
                .await;
        });
    });

    view! {ctx,
    div{
        video(
            ref=video_ref,
            style="border:1px solid red",
            autoplay=false,
            width=640,
            height=480,
            src="https://imgs-qn.51miz.com/preview/video/00/00/14/33/V-143360-BCE1F72B.mp4",
        )
    }
    }
}
