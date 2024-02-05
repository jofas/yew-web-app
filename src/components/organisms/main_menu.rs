use crate::get_demo_videos;
use crate::get_toggle_key;
use crate::Route;
use crate::VideosList;
use yew::prelude::*;
use yew_router::prelude::use_navigator;
use std::cell::RefCell;

#[function_component(MainMenu)]
pub fn main_menu() -> Html {
    let demo_videos = get_demo_videos();
    // State to track the index of the currently displayed demo video
    //let current_video_index = use_state(|| 0);
    let current_video_index = use_state(|| RefCell::new(0));
    let current_video_index_clone = current_video_index.clone();

    //Handle keydown events to switch videos
    let handle_keydown_toggle = get_toggle_key(&demo_videos, current_video_index.clone());
    let navigator = use_navigator().unwrap();
    let press_r_for_about = Callback::from(move |event: KeyboardEvent| {
        if event.key() == "r" {
            let current_index = current_video_index_clone.borrow().clone();
            match current_index {
                1 => navigator.push(&Route::AboutChoreo1),
                2 => navigator.push(&Route::AboutChoreo2),
                3 => navigator.push(&Route::AboutChoreo3),
                4 => navigator.push(&Route::AboutChoreo4),
                _ => {}
            }
            let soundeffect =
                web_sys::HtmlAudioElement::new_with_src("static/buttonClick.mp3").unwrap();
            let _ = soundeffect.play();
        }
    });

    html! {

        <div onkeydown={press_r_for_about} tabindex="0">
            <audio src={format!("static/8bit-menusong-short-ed.aif")} autoplay=true loop=true />
            <div onkeydown={handle_keydown_toggle} tabindex="0">
                <VideosList videos={demo_videos} current_index={current_video_index.clone()} />
                <img src="static/danceOmatic_logo.png" alt="logo of danceomatic"/>
            </div>
        </div>

    }
}
