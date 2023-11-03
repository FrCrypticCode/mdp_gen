use eframe::egui::*;

mod lib;
use lib::make_mdp;

struct MyApp{
    nb: String,
    res: String,
    alert: String
}

fn main()->eframe::Result<()>{
    let options = eframe::NativeOptions{
        transparent:true,
        initial_window_size:(Some(egui::vec2(260.0,160.0))),
        ..Default::default()
    };

    let mut app = MyApp{
        nb:String::new(),
        res:String::new(),
        alert:String::new()
    };

    eframe::run_simple_native("Mdp_Gen",options, move |ctx, frame|{
        egui::CentralPanel::default().show(ctx,|ui|{
            ui.label("Renseignez le nombre de caractère :");
            ui.add(TextEdit::singleline(&mut app.nb));
            ui.horizontal(|ui|{
                ui.label("Votre Mdp :");
                ui.label(&app.res);
            });
            ui.label(&app.alert);
            ui.vertical_centered(|ui|{
                if ui.button("Générer").clicked(){
                    if let Ok(x) = app.nb.trim().parse::<u16>(){
                        app.res = make_mdp(x);
                        app.alert = String::from("");
                    }
                    else{
                        app.alert = String::from("Vous devez renseigner un nombre.");
                    }
                }
            });
            ui.vertical_centered(|ui|{
                if ui.button("Quit").clicked(){
                    frame.close();
                }
            });
        });
    })
}