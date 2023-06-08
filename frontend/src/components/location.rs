use yew::prelude::*;
use yew_hooks::use_geolocation;

#[function_component(GeoLocation)]
pub fn geo_location() -> Html {
    let location = use_geolocation();
    html!(
        <>
            <span class="input-group-text" title={"Latitude"}>
              <i class="fa-regular fa-map fa-rotate-90"></i>
            </span>
            <div class="form-floating">
                <input
                    class="form-control"
                    type="text"
                    id="latitudeGroup"
                    value={location.latitude.to_string()}
                    />
                <label for="latitudeGroup">{"Latitude"}</label>
            </div>
            <span class="input-group-text" title={"Longitude"}>
              <i class="fa-regular fa-map"></i>
            </span>
            <div class="form-floating">
                <input
                    class="form-control"
                    type="text"
                    id="longitudeGroup"
                    value={location.longitude.to_string()}
                    />
                <label for="longitudeGroup">{"Longitude"}</label>
            </div>
        </>
    )
}

#[function_component(Location)]
pub fn location() -> Html {
    html!(
        <>
            <span class="input-group-text">
              <i class="fa-solid fa-location-dot"></i>
            </span>
            <div class="form-floating">
                <input
                    class="form-control"
                    type="text"
                    id="LocationGroup"
                    placeholder="Location"
                    />
                <label for="LocationGroup">{"Location"}</label>
            </div>
        </>
    )
}