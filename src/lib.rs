mod pages;
mod api;
mod types;
mod components;

use pages::Home;
use wasm_bindgen::prelude::*;
use yew::prelude::*;


#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Home>::new().mount_to_body();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
