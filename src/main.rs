use gtk::{ Button, ButtonExt, ContainerExt, Inhibit, Label, LabelExt, Orientation, WidgetExt, Window};
use relm::{connect, Relm, Update, Widget};
use relm_derive::Msg;

struct Model {}

#[derive(Msg)]
enum Msg {
    Hello,
    Quit,
}

#[derive(Clone)]
struct Widgets {
    lbl: Label,
    but: Button,
    window: Window,
}

#[allow(dead_code)]
struct Win {
    model: Model,
    widgets: Widgets,
}

impl Update for Win {
    type Model = Model;
    type ModelParam = ();
    type Msg = Msg;

    fn model(_: &Relm<Self>, _: ()) -> Model {
        Model {}
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::Hello => { &self.widgets.lbl.set_text("Hello, World!"); },
            Msg::Quit => gtk::main_quit(),
        }
    }
}

impl Widget for Win {
    type Root = Window;

    fn root(&self) -> Self::Root {
        self.widgets.window.clone()
    }

    fn view(relm: &Relm<Self>, model: Self::Model) -> Self {
        // Create the view using the normal GTK+ method calls.
        let window = Window::new(gtk::WindowType::Toplevel);

        let lbl = Label::new(None);
        let but = Button::with_label("Click Me");

        let vbox = gtk::Box::new(Orientation::Vertical, 0);
        vbox.add(&lbl);
        vbox.add(&but);

        window.add(&vbox);
        window.show_all();

        connect!(relm, but, connect_clicked(_), Msg::Hello);
        connect!(relm, window, connect_delete_event(_, _), return (Some(Msg::Quit), Inhibit(false)));

        Win {
            model,
            widgets: Widgets {
                lbl,
                but,
                window,
            },
        }
    }
}

fn main() {
    Win::run(()).expect("Win::run failed");
}