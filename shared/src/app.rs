use crux_core::render::Render;
use crux_macros::Effect;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Event {
    None,
}

#[derive(Default)]
pub struct Model;

#[derive(Serialize, Deserialize)]
pub struct ViewModel {
    name: String,
    email: String
}

#[cfg_attr(feature = "typegen", derive(crux_macros::Export))]
#[derive(Effect)]
#[effect(app = "App")]
pub struct Capabilities {
    render: Render<Event>,
}

#[derive(Default)]
pub struct App;

impl crux_core::App for App {
    type Event = Event;
    type Model = Model;
    type ViewModel = ViewModel;
    type Capabilities = Capabilities;

    fn update(&self, _event: Self::Event, _model: &mut Self::Model, caps: &Self::Capabilities) {
        caps.render.render();
    }

    fn view(&self, _model: &Self::Model) -> Self::ViewModel {
        ViewModel {
            name: "Jon Yardley".to_string(),
            email: "jonyardley@me.com".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crux_core::{assert_effect, testing::AppTester};

    #[test]
    fn user_prints_name_and_email() {
        let app = AppTester::<App, _>::default();
        let mut model = Model::default();

        let update = app.update(Event::None, &mut model);
        assert_effect!(update, Effect::Render(_));

        let name = &app.view(&model).name;
        let email = &app.view(&model).email;
        assert_eq!(name, "Jon Yardley");
        assert_eq!(email, "jonyardley@me.com");
    }
}
