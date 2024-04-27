mod pr_file_format;
mod cursive_methods;
mod globals;

use std::path::PathBuf;

use cursive_methods::*;
use pr_file_format::PRSeason;
use globals::PR_SEASON;

use cursive::{
    traits::*, views::{Dialog, ListView, Panel, SelectView, TextView}, Cursive
};

fn update_main_menu_loaded_season(ui: &mut Cursive) {
    ui.call_on_name("Summary Text", |mm: &mut TextView| {
        let season = PR_SEASON.lock().unwrap();
        mm.set_content(
            format!("Current Season:\nNumber of players: {}\nNumber of Brackets: {}",
                season.get_num_players(),
                season.get_num_brackets()
            )
        );
    });
}

fn main_menu_options_loaded_season(ui: &mut Cursive) {
    ui.call_on_name("Main Menu List", |mm: &mut SelectView| {
        mm.clear();
        mm.set_on_submit(|s: &mut Cursive, val: &str| {
            match val {
                "Get Bracket from Start.GG" => {},
                "Alter Player Information" => {
                    s.add_layer(create_alter_player_list());
                    update_player_list(s);
                },
                "Alter Bracket Information" => {},
                "View Standings" => {},
                "Save Season" => {
                    s.add_layer(create_save_file_menu(|_s: &mut Cursive, val: PathBuf| {
                        let mut season = PR_SEASON.lock().unwrap();
                        season.save_to_file(val.clone()).unwrap();
                        drop(season);
                    }));
                },
                "Close Season" => {
                    s.add_layer(Dialog::new()
                        .button("Cancel", |s| { s.pop_layer(); })
                        .button("Yes", |s| {
                            let mut season = PR_SEASON.lock().unwrap();
                            *season = PRSeason::new();
                            drop(season);
                            update_main_menu_no_season(s);
                            main_menu_options_no_season(s);
                            s.pop_layer();
                        })
                        .content(TextView::new("Are you sure you want to close?\nYou will lose any unsaved progress!"))
                        .title("Are you sure?")
                    );
                },
                _ => {}
            }
        });

        mm.add_all_str(
            vec!["Get Bracket from Start.GG", "Alter Player Information", "Alter Bracket Information",
            "View Standings", "Save Season", "Close Season"]
        );
    });
}

fn update_main_menu_no_season(ui: &mut Cursive) {
    ui.call_on_name("Summary Text", |mm: &mut TextView| {
        mm.set_content("No Currently Loaded Season.");
    });
}

fn main_menu_options_no_season(ui: &mut Cursive) {
    ui.call_on_name("Main Menu List", |mm: &mut SelectView| {
        mm.clear();
        mm.set_on_submit(|s, val: &str| {
            match val {
                "New Season" => {
                    let mut season = PR_SEASON.lock().unwrap();
                    *season = PRSeason::new();
                    drop(season);

                    update_main_menu_loaded_season(s);
                    main_menu_options_loaded_season(s);
                }
                "Load Season" => { s.add_layer(cursive_methods::create_load_file_menu(|s, path| {
                        let mut season = PR_SEASON.lock().unwrap();
                        *season = match PRSeason::load_from_file(path) {
                            Ok(pr) => pr,
                            Err(e) => panic!("{}", e)
                        };
                        drop(season);
                        s.pop_layer();
                        update_main_menu_loaded_season(s);
                        main_menu_options_loaded_season(s);
                    }));
                },
                "Quit" => { s.quit(); }
                _ => {}
            };
        });
        mm.add_item_str("New Season");
        mm.add_item_str("Load Season");
        mm.add_item_str("Quit");
    });
}

fn main() {
    let main_menu = ListView::new()
        .child("", TextView::new("No Currently Loaded Season.").with_name("Summary Text"));
    
    let main_menu_selector: SelectView<String> = SelectView::new().h_align(cursive::align::HAlign::Center);

    let main_menu_selector = Panel::new(main_menu_selector.with_name("Main Menu List"));
    let main_menu = main_menu.child("", main_menu_selector).with_name("Main Menu");

    let mut ui = cursive::default();

    ui.add_layer(Dialog::new()
        .content(main_menu)
        .title("RPI's PR Software")
    );

    main_menu_options_no_season(&mut ui);
    update_main_menu_no_season(&mut ui);

    ui.run();
}
