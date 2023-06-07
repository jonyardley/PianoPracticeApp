use crux_core::render::Render;
use crux_macros::Effect;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Event {
    SetName(String),
    SetEmail(String),
    Reset,
}

#[derive(Default)]
pub struct Model {
    name: String,
    email: String
}

#[derive(Serialize, Deserialize)]
pub struct ViewModel {
    pub name: String,
    pub email: String
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

    fn update(&self, msg: Self::Event, model: &mut Self::Model, caps: &Self::Capabilities) {
        match msg {
            Event::SetName(name) => model.name = name,
            Event::SetEmail(email) => model.email = email,
            Event::Reset => {
                model.email = "".to_string();
                model.name = "".to_string();
            }
        }
        caps.render.render();
    }

    fn view(&self, model: &Self::Model) -> Self::ViewModel {
        ViewModel {
            name: model.name.to_string(),
            email: model.email.to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crux_core::{assert_effect, testing::AppTester};

    #[test]
    fn reset_prints_default_model() {
        let app = AppTester::<App, _>::default();
        let mut model = Model::default();

        let update = app.update(Event::Reset, &mut model);
        assert_effect!(update, Effect::Render(_));

        let name = &app.view(&model).name;
        let email = &app.view(&model).email;
        assert_eq!(name, "");
        assert_eq!(email, "");
    }

    #[test]
    fn sets_name() {
        let app = AppTester::<App, _>::default();
        let mut model = Model::default();

        let new_name = "New Name";

        let update = app.update(Event::SetName(new_name.to_string()), &mut model);
        assert_effect!(update, Effect::Render(_));

        let name = &app.view(&model).name;
        assert_eq!(name, new_name);
    }

    #[test]
    fn sets_email() {
        let app = AppTester::<App, _>::default();
        let mut model = Model::default();

        let new_email = "new@email.com";

        let update = app.update(Event::SetEmail(new_email.to_string()), &mut model);
        assert_effect!(update, Effect::Render(_));

        let email = &app.view(&model).email;
        assert_eq!(email, new_email);
    }
}
