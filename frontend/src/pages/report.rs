use web_sys::HtmlInputElement;
use crate::components::location::Location;
use crate::components::location::GeoLocation;
use yew::prelude::*;
use yew_hooks::{use_counter, use_geolocation};

#[function_component(Report)]
pub fn report() -> Html {
    let drag_over = use_counter(0);
    let gps = use_state(|| false);

    let gps_enabled = *gps;
    let onclick_gps = { Callback::from(move |_| gps.set(!*gps)) };

    let on_image_select = {
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
        })
    };

    let on_image_drop = {
        let drag_over = drag_over.clone();
        Callback::from(move |e: DragEvent| {
            e.prevent_default();
            drag_over.set(0);
        })
    };

    let on_drag_enter = {
        let drag_over = drag_over.clone();
        Callback::from(move |e: DragEvent| {
            e.prevent_default();
            if let Some(input) = e.data_transfer() {
                drag_over.increase();
            };
        })
    };

    let on_drag_leave = {
        let drag_over = drag_over.clone();
        Callback::from(move |e: DragEvent| {
            if let Some(input) = e.data_transfer() {
                drag_over.decrease();
            };
        })
    };


    let drag_over_class = if (*drag_over) > 0 {
        Some("btn-secondary")
    } else {
        Some("btn-outline-secondary")
    };

    let file_picker = use_node_ref();
    let f_picker = file_picker.clone();
    let click_add_image = Callback::from(move |_| {
        if let Some(element) = f_picker.cast::<HtmlInputElement>() {
            element.click();
        };
    });

    html!(
        <section class="hero is-danger is-bold is-large">
            <div class="hero-body">
                <div class="container">
                    <div class="input-group mb-2">
                        <span class="input-group-text">
                          <i class="fa-regular fa-id-card"></i>
                        </span>
                        <div class="form-floating">
                            <input
                                class="form-control"
                                type="text"
                                id="SPZGroup"
                                placeholder="SPZ"
                                />
                            <label for="SPZGroup">{"SPZ"}</label>
                        </div>
                    </div>
                    <div class="input-group mb-2">
                        if gps_enabled {
                            <GeoLocation />
                        } else {
                            <Location />
                        }
                        <button type="button" class="btn btn-secondary btn-lg" onclick={onclick_gps}>
                            if gps_enabled {
                                <i class="fa-solid fa-signature"></i>
                            } else {
                                <i class="fa-solid fa-satellite-dish"></i>
                            }
                        </button>

                    </div>
                    <div class="input-group mb-2">
                        <span class="input-group-text" title={"When is the report dated"}>
                          <i class="fa-regular fa-calendar-check"></i>
                        </span>
                        <div class="form-floating">
                            <input
                                class="form-control"
                                type="datetime-local"
                                id="dateGroup"
                                />
                            <label for="dateGroup">{"Date"}</label>
                        </div>
                    </div>
                    <div class="input-group mb-2">
                        <div class="form-floating">
                          <textarea class="form-control" id="floatingTextarea2" style="height: 100px"></textarea>
                          <label for="floatingTextarea2">{"Description"}</label>
                        </div>
                    </div>
                    <div>
                        <div class="h5">
                            {"Pictures"}
                        </div>
                        <button type="button" class={classes!("btn", drag_over_class, "btn-lg", "p-2", "w-100")} onclick={click_add_image}
                                ondrop={on_image_drop}
                                ondragover={|e: DragEvent| e.prevent_default() }
                                ondragleave={on_drag_leave}
                                ondragenter={on_drag_enter}>{"Add image "}<i class="fa-regular fa-image fa-beat"></i></button>
                        <input ref={file_picker} type="file" accept="image/jpeg" style="display:none;" onchange={on_image_select} multiple={true}/>
                    </div>
                </div>
            </div>
        </section>
    )
}
