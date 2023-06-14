use eframe::App;
use eframe::egui::{Color32, self, Window, Button};
use eframe::glow::TIME_ELAPSED;
use tokio::{io, time::Instant};
use tokio::task;
use tokio:: runtime::Runtime;
use std::{path::PathBuf};
use crate::request::{RequestType, Response,self, my_files, my_folders, files};
use egui_extras::RetainedImage;

struct Style
{
    side_panel: Color32, 
    main_panel: Color32, 
    color_panel: Color32, 
    button_color_panel: Color32,
    color: Color32, 
    button_color: Color32,
    image:RetainedImage,
}
// Enum to represent different pages/screens of the application
enum Page {
    Signup,
    Login,
    Home,
    MyFiles,
    SharedFiles,
    FolderDetails,
    Splash,
}

// Define an enum to represent the current theme/mode
enum Theme 
{
    Dark,
    Light,
}

impl Default for Theme 
{
    fn default() -> Self 
    {
        Theme::Light // Set the default theme/mode to dark
    }
}

pub struct Gui 
{
    splash:RetainedImage,
    signup_bg: RetainedImage,
    login_bg: RetainedImage,

    selected_file:String,
    share_window : bool,
    remove_item : Option<String>,
    selected_folder: String,
    selected_path: String,
    uploadList: Vec<my_files>,
    foldernames: Vec<my_folders>,
    selected_folderlist : Vec<files>,
    // Current page of the application
    current_page: Page,

    gui_style:Style,

    // Error message to display
    error_message: Option<String>,

    // Data for login and signup page
    username: String,
    email:String,
    password: String,
    confirm_password: String,
    emails_input : String,
    share_with : Vec<String>,

    // Extra data
    theme: Theme,
    timer: Instant,
}

impl Default for Gui
{
    fn default() -> Self 
    {
        Self 
        {
            timer: Instant::now(),
            signup_bg: RetainedImage::from_image_bytes(
                "bg_signup",
                include_bytes!("bg_signup.png"),
            )
            .unwrap(),
            login_bg: RetainedImage::from_image_bytes(
                "bg_login",
                include_bytes!("bg_login.png"),
            )
            .unwrap(),
            splash : RetainedImage::from_image_bytes(
                "bg_login",
                include_bytes!("splash.png"),
            )
            .unwrap(),

            selected_file: String::new(),
            share_window : false,
            remove_item:None,
            current_page: Page::Splash,
            selected_folder: String::new(),
            selected_path : String::new(),
            selected_folderlist : Vec::new(),
            uploadList : Vec::new(), 
            foldernames : Vec::new(),
            error_message: None,
            username: String::new(),
            email:String::new(),
            password: String::new(),
            confirm_password: String::new(),
            emails_input : String::new(),
            share_with : Vec::new(),
            theme: Theme::default(),
            gui_style: Style { side_panel: (Color32::from_rgb(3, 97, 216)), main_panel: (Color32::WHITE), color_panel: (Color32::WHITE), button_color_panel: (Color32::from_rgb(3, 97, 216)),  color: Color32::BLACK, button_color: Color32::from_rgb(3, 97, 216), image: RetainedImage::from_image_bytes("bg_light",include_bytes!("bg_light.jpg"),).unwrap() }
        }
    }
}

impl Gui {
    fn toggle_theme(&mut self, ctx: &egui::Context) {
        match self.theme {
            Theme::Dark => {
                self.theme = Theme::Light;
                ctx.set_visuals(egui::Visuals::light());
                self.gui_style = Style { side_panel: (Color32::from_rgb(3, 97, 216)), main_panel: (Color32::WHITE), color_panel: (Color32::WHITE), button_color_panel: (Color32::from_rgb(3, 97, 216)), color: Color32::BLACK, button_color: Color32::from_rgb(3, 97, 216), image: RetainedImage::from_image_bytes("bg_light",include_bytes!("bg_light.jpg"),).unwrap() }
            }
            Theme::Light => {
                self.theme = Theme::Dark;
                ctx.set_visuals(egui::Visuals::dark());
                self.gui_style = Style { side_panel: (Color32::from_rgb(24, 41, 63)), main_panel: (Color32::from_rgb(60, 77, 104)), color_panel: (Color32::WHITE), button_color_panel: (Color32::from_rgb(24, 41, 63)), color: Color32::WHITE, button_color: Color32::from_rgb(3, 97, 216), image: RetainedImage::from_image_bytes("bg_dark",include_bytes!("bg_dark.jpg"),).unwrap()}
            }
        }
    }

    fn splash(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) 
    {
        let mut rect =   ctx.screen_rect();
        ui.image(self.splash.texture_id(ctx), rect.size());
        if self.timer.elapsed().as_secs_f32() > 7.0
        {
            self.current_page = Page::Login;
        }
    }

    // Function to show the signup page UI
    fn signup_page(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) 
    {
        _ctx.set_visuals(egui::Visuals::light());
        let frame1 = egui::containers::Frame 
        {
            shadow: egui::epaint::Shadow{ extrusion: 0.0, color: Color32::TRANSPARENT },
            fill: Color32::from_rgb(14 ,43, 64),
            stroke: egui::Stroke::new(0.0, Color32::TRANSPARENT),
            inner_margin: egui::Margin::default(),
            outer_margin: egui::Margin::default(),
            rounding: egui::Rounding::none(),
        };
        let frame2 = egui::containers::Frame 
        {
            shadow: egui::epaint::Shadow{ extrusion: 0.0, color: Color32::TRANSPARENT },
            fill: Color32::TRANSPARENT,
            stroke: egui::Stroke::new(0.0, Color32::TRANSPARENT),
            inner_margin: egui::Margin::default(),
            outer_margin: egui::Margin::default(),
            rounding: egui::Rounding::same(60.0),
        };
        egui::CentralPanel::default().show(_ctx, |ui| 
        {
            egui::SidePanel::right("signup")
            .show_separator_line(false)
            .frame(frame1)
            .min_width(400.0)
            .resizable(false)
            .show(_ctx, |ui| 
            {
                ui.add_space(ui.available_height() / 5.0);
                ui.vertical_centered(|ui| 
                {  
                    ui.add_space(25.0);
                    ui.heading(egui::RichText::new("SIGNUP").color(Color32::WHITE).strong());
                    ui.style_mut() .visuals .widgets .noninteractive .bg_stroke .color = egui::Color32::TRANSPARENT;
                    ui.group(|ui| 
                    {
                        //ui.reset_style();
                        ui.set_max_width(300.0);
                        ui.vertical(|ui| 
                        {
                            ui.label(egui::RichText::new("Name").color(Color32::WHITE));
                            ui.text_edit_singleline(&mut self.username);
                            ui.add_space(5.0);
                            ui.label(egui::RichText::new("Email").color(Color32::WHITE));
                            ui.text_edit_singleline(&mut self.email);
                            ui.add_space(5.0);
                            ui.label(egui::RichText::new("Password").color(Color32::WHITE));
                            ui.text_edit_singleline(&mut self.password);
                            ui.add_space(5.0);
                            ui.label(egui::RichText::new("Confirm Password").color(Color32::WHITE));
                            ui.text_edit_singleline(&mut self.confirm_password);
                            ui.add_space(25.0);
                            ui.vertical_centered(|ui| 
                            {
                                ui.style_mut().visuals.widgets.inactive.weak_bg_fill = Color32::from_rgb(61, 100, 255);
                                ui.style_mut().visuals.widgets.inactive.fg_stroke = egui::Stroke{ width: 300.0, color: egui::Color32::WHITE };
                                ui.style_mut().spacing.button_padding = egui::vec2(70.0, 3.0);
                                if ui.button("SIGNUP").clicked() 
                                {
                                    self.error_message  = None;
                                    // Handle signup button click
                                    if self.password != self.confirm_password {
                                        // Show an error message if the passwords don't match
                                        self.error_message = Some("Passwords don't match".to_string()); // Store error message in a variable
                                    } else if self.username.is_empty() || self.password.is_empty() || self.email.is_empty(){
                                        // Show an error message if either the username or password is empty
                                        self.error_message = Some("All fields required".to_string()); // Store error message in a variable
                                    }else if !self.email.contains('@'){
                                        // Show an error message if either the username or password is empty
                                        self.error_message = Some("Invalid email".to_string()); // Store error message in a variable
                                    }
                                    else 
                                    {
                                        let rt= Runtime::new().unwrap();
                                        let username= self.username.clone();
                                        let email= self.email.clone();
                                        let pass= self.password.clone();
                                        rt.block_on( async
                                        {
                                            let result = tokio::spawn(async move
                                            {
                                                let request:RequestType = RequestType::Signup { name:username, password:pass};
                                                request::new(request, &email).await
                                            }).await.unwrap();
                                            match result
                                            {
                                                Response::Success => self.current_page = Page::Login,
                                                Response::Failure(e) => self.error_message = Some(e.clone()),
                                                _ => {},
                                            }
                                        }); 
                                    }
                                }
                                if let Some(error_message) = &self.error_message 
                                {
                                    // Display error message if it exists
                                    ui.add_space(5.0);
                                    ui.label(egui::RichText::new(error_message).color(egui::Color32::RED));
                                }
                                ui.add_space(10.0);
                                ui.separator();
                                ui.add_space(10.0);
                                if ui.button("LOGIN").clicked() 
                                {
                                    // Show the signup page if the user clicks the sign up button
                                    self.error_message = None;
                                    self.username.clear();
                                    self.password.clear();
                                    self.current_page = Page::Login;
                                }
                            });
                        });
                    })
                });
            });
            egui::SidePanel::left("background")
            .min_width(ui.available_width())
            .frame(frame2)
            .show_separator_line(false)
            .show(_ctx, |ui| 
            {
                let rect = _ctx.available_rect();
                ui.image(self.signup_bg.texture_id(_ctx), rect.size());                
            });
        });
    }

    // Function to show the login page UI
    fn login_page(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) 
    {
        let mut rect =   _ctx.screen_rect();
        ui.image(self.login_bg.texture_id(_ctx), rect.size());
        _ctx.set_visuals(egui::Visuals::light());
        let frame = egui::containers::Frame 
        {
            shadow: egui::epaint::Shadow{ extrusion: 0.0, color: Color32::TRANSPARENT },
            fill: Color32::TRANSPARENT,
            stroke: egui::Stroke::new(0.0, Color32::TRANSPARENT),
            inner_margin: egui::Margin::same(0.0),
            outer_margin: egui::Margin::same(0.0),
            rounding: egui::Rounding::none(),
        };
        egui::CentralPanel::default().frame(frame).show(_ctx, |ui| 
        {
            ui.add_space(ui.available_height() / 4.0);
            ui.vertical_centered_justified(|ui| 
            {

               // ui.label(egui::RichText::new("ShareSphere |  Share. Collaborate. Thrive.").strong().italics().color(Color32::WHITE).text_style(egui::TextStyle::Heading));
                ui.add_space(45.0);
                ui.heading(egui::RichText::new("LOGIN").color(Color32::BLACK).strong());
                ui.style_mut() .visuals .widgets .noninteractive .bg_stroke .color = egui::Color32::TRANSPARENT;
                ui.group(|ui| 
                {
                    ui.reset_style();
                    ui.set_max_width(300.0);
                    ui.vertical(|ui| 
                    {
                        ui.label(egui::RichText::new("Email").color(Color32::BLACK));
                        ui.text_edit_singleline(&mut self.email);
                        ui.add_space(5.0);
                        ui.label(egui::RichText::new("Password").color(Color32::BLACK));
                        ui.text_edit_singleline(&mut self.password);
                        ui.add_space(25.0);
                        ui.vertical_centered(|ui| 
                        {
                            ui.style_mut().visuals.widgets.inactive.weak_bg_fill = egui::Color32::from_rgb(61, 100, 255);
                            ui.style_mut().visuals.widgets.inactive.fg_stroke = egui::Stroke{ width: 300.0, color: egui::Color32::WHITE };
                            ui.style_mut().spacing.button_padding = egui::vec2(70.0, 3.0);
                            if ui.button("LOGIN").clicked() 
                            {   
                                self.error_message = None;
                                if self.email.is_empty() || self.password.is_empty()
                                {
                                    // Show an error message if either the username or password is empty
                                    self.error_message = Some("Enter email/password".to_string()); // Store error message in a variable
                                }
                                else if !self.email.contains('@')
                                {
                                    // Show an error message if either the username or password is empty
                                    self.error_message = Some("Invalid email".to_string()); // Store error message in a variable
                                }
                                else 
                                {
                                    let rt= Runtime::new().unwrap();
                                    let email= self.email.clone();
                                    let pass= self.password.clone();
                                    rt.block_on( async
                                    {
                                        let result = tokio::spawn(async move
                                        {
                                            let request:RequestType = RequestType::Login { password: (pass) };
                                            request::new(request, &email).await
                                        }).await.unwrap();
                                        match result
                                        {
                                            Response::Success => self.current_page = Page::Home,
                                            Response::Failure(e) => self.error_message = Some(e.clone()),
                                            _ => {},
                                        }
                                    }); 
                                }
                            }
                            if let Some(error_message) = &self.error_message 
                            {
                                // Display error message if it exists
                                ui.add_space(5.0);
                                ui.label(egui::RichText::new(error_message).color(egui::Color32::RED));
                            }
                            ui.add_space(10.0);
                            ui.separator();
                            ui.add_space(10.0);
                            if ui.button("SIGNUP").clicked() 
                            {
                                // Show the signup page if the user clicks the sign up button
                                self.error_message = None;
                                self.username.clear();
                                self.password.clear();
                                self.current_page = Page::Signup;
                            }
                        });
                    });
                })
            });

        });
    }

    // Function to show the home page UI
    fn home_page(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) 
    {
        let frame1 = egui::containers::Frame 
        {
            shadow: egui::epaint::Shadow{ extrusion: 0.0, color: Color32::TRANSPARENT },
            fill: self.gui_style.side_panel,
            stroke: egui::Stroke::new(0.0, Color32::TRANSPARENT),
            inner_margin: egui::Margin::default(),
            outer_margin: egui::Margin::default(),
            rounding: egui::Rounding::none(),
        };
        let frame2 = egui::containers::Frame 
        {
            shadow: egui::epaint::Shadow{ extrusion: 0.0, color: Color32::TRANSPARENT },
            fill: Color32::TRANSPARENT,
            stroke: egui::Stroke::new(0.0, Color32::TRANSPARENT),
            inner_margin: egui::Margin::default(),
            outer_margin: egui::Margin::default(),
            rounding: egui::Rounding::none(),
        };
        let frame3 = egui::containers::Frame 
        {
            shadow: egui::epaint::Shadow{ extrusion: 0.0, color: Color32::TRANSPARENT },
            fill: Color32::TRANSPARENT,
            stroke: egui::Stroke::new(0.0, Color32::TRANSPARENT),
            inner_margin: egui::Margin::default(),
            outer_margin: egui::Margin::default(),
            rounding: egui::Rounding::none(),
        };
        let rect = ctx.available_rect();
        ui.image(self.gui_style.image.texture_id(ctx), rect.size());
        egui::CentralPanel::default().frame(frame3).show(ctx, |ui| 
        {
                egui::SidePanel::left("side_panel1")
                .show_separator_line(false)
                .frame(frame1)
                .min_width(200.0)
                .resizable(false)
                .show(ctx, |ui| 
                {
                    ui.style_mut().visuals.widgets.inactive.weak_bg_fill = self.gui_style.button_color_panel;
                    ui.style_mut().visuals.widgets.inactive.fg_stroke = egui::Stroke{ width: 300.0, color: self.gui_style.color_panel };
                    ui.style_mut().spacing.button_padding = egui::vec2(0.0, 0.0);
                    ui.add_space(15.0);
                    ui.horizontal(|ui|{ui.add_space(10.0);ui.label(egui::RichText::new("ShareSphere").italics().heading().color(self.gui_style.color_panel))});
                    
                    ui.vertical_centered_justified(|ui|
                    {
                        ui.add_space(20.0);
                        if  ui.button("HOME").clicked() {
                            self.error_message = None;
                            self.current_page = Page::Home;
                            
                        }
                        ui.add_space(20.0);
                        if ui.button("MY UPLOADS").clicked() 
                        {
                            self.error_message = None;
                            let rt= Runtime::new().unwrap();
                            let email= self.email.clone();
                            rt.block_on( async
                            {
                                let result = tokio::spawn(async move
                                {
                                    let request:RequestType = RequestType::MyUploadList;
                                    request::new(request, &email).await
                                }).await.unwrap();
                                match result
                                {
                                    Response::MyUploadListSuccess {list } => self.uploadList = list,
                                    Response::Failure(e) => self.error_message = Some(e.clone()),
                                    _ => {},
                                }
                            }); 
                            // Redirect to My Files page
                            self.current_page = Page::MyFiles;
                        }
                        ui.add_space(20.0);
                        if ui.button("SHARED WITH ME").clicked() 
                        {
                            self.error_message = None;
                            let rt= Runtime::new().unwrap();
                            let email= self.email.clone();
                            rt.block_on( async
                            {
                                let result = tokio::spawn(async move
                                {
                                    let request:RequestType = RequestType::GetFolderNames;
                                    request::new(request, &email).await
                                }).await.unwrap();
                                match result
                                {
                                    Response::GetFolderNameSuccess { foldernames } => self.foldernames = foldernames,
                                    Response::Failure(e) => self.error_message = Some(e.clone()),
                                    _ => {},
                                }
                            }); 
                            // Redirect to Shared Files page
                            self.current_page = Page::SharedFiles;
                        }
                        ui.add_space(20.0);
                        if ui.button("THEME").clicked() {
                            self.error_message = None;
                            self.toggle_theme(ctx);
                        }
                        ui.add_space(20.0);
                        if ui.button("LOGOUT").clicked() {
                            // Redirect to Login page and clear user data
                            self.username.clear();
                            self.password.clear();
                            self.confirm_password.clear();
                            self.error_message = None;
                            self.toggle_theme(ctx);
                            self.current_page = Page::Login;
                        }
                    });
                });
                egui::SidePanel::left("side_panel2")
                .min_width(ui.available_width())
                .frame(frame2)
                .show(ctx, |ui| 
                {
                    if let Page::MyFiles = self.current_page{self.my_files_page(ctx, ui);}
                    else if let Page::SharedFiles = self.current_page{self.shared_files_page(ctx, ui);}
                    else if let Page::FolderDetails = self.current_page 
                    {
                        let folder = self.selected_folder.clone();
                        self.display_folder(ctx, ui, &folder);
                    }
                    else{self.upload(ctx, ui);}
                });
        });
    }

    // Function to show the My Files page UI
    fn my_files_page(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) 
    {
        ui.vertical(|ui|
        {
            ui.add_space(15.0);
            ui.horizontal(|ui| 
            {            
                ui.add_space(10.0);
                ui.heading(egui::RichText::new("MY UPLOADS").color(self.gui_style.color)); // Display heading "My Files"
            });
            ui.add_space(30.0)
        });
        // egui::ScrollArea::auto_sized().show(ui, |ui| 
        // {
                ui.horizontal(|ui|
                {
                    ui.add_space(15.0);
                    egui::Grid::new("my_grid").striped(true).num_columns(5).min_col_width(ui.available_width()/9.0).max_col_width(ui.available_width()/4.0).spacing(egui::vec2(30.0, 10.0)).show(ui, |ui| 
                    {
                        ui.label(egui::RichText::new("Title").color(self.gui_style.color).text_style(egui::TextStyle::Body).strong());
                        ui.label(egui::RichText::new("Access").color(self.gui_style.color).text_style(egui::TextStyle::Body).strong());
                        ui.label(egui::RichText::new("Modified").color(self.gui_style.color).text_style(egui::TextStyle::Body).strong());
                        ui.label(egui::RichText::new("Size").color(self.gui_style.color).text_style(egui::TextStyle::Body).strong()); 
                        ui.end_row();
                        if self.uploadList.is_empty(){ui.label(egui::RichText::new("No items to show").color(self.gui_style.color));}
                        for row in  &self.uploadList
                        {
                            let members = match row.members.clone()
                            {
                                Some(value) => 
                                {
                                    value.replace(",", "\n")
                                },
                                None => "Private".to_string(),
                            };
                            ui.label(egui::RichText::new(row.name.clone()).color(self.gui_style.color));
                            ui.label(egui::RichText::new(members).color(self.gui_style.color));
                            ui.label(egui::RichText::new(row.upload_date.clone()).color(self.gui_style.color));
                            ui.label(egui::RichText::new(row.size.to_string()).color(self.gui_style.color));
                            ui.horizontal(|ui|
                            {
                                ui.style_mut().visuals.widgets.inactive.weak_bg_fill = self.gui_style.button_color;
                                ui.style_mut().visuals.widgets.inactive.fg_stroke = egui::Stroke{ width: 300.0, color: Color32::WHITE };
                                if ui.button("Download").clicked() 
                                {
                                    let name = row.name.clone();
                                    if let Some(path) = rfd::FileDialog::new().pick_folder()
                                    {
                                        let filepath = path.display().to_string();
                                        let rt= Runtime::new().unwrap();
                                        let email= self.email.clone();
                                        rt.block_on( async
                                        {
                                            let result = tokio::spawn(async move
                                            {
                                                let request:RequestType = RequestType::DownloadFile { file_name: (name), file_path: filepath };
                                                request::new(request, &email).await
                                            }).await.unwrap();
                                            match result
                                            {
                                                Response::Success => 
                                                {},
                                                Response::Failure(e) => self.error_message = Some(e.clone()),
                                                _ => {},
                                            }
                                        }); 
                                    }
                                }
                                if ui.button("Share").clicked() 
                                {
                                    self.selected_file = row.name.clone();
                                    self.share_window = true;
                                }
                                if ui.button("Remove").clicked() 
                                {
                                    let rt= Runtime::new().unwrap();
                                    let email= self.email.clone();
                                    let name = row.name.clone();
                                    let remove_name = row.name.clone();
                                    rt.block_on( async
                                    {
                                        let result = tokio::spawn(async move
                                        {
                                            let request:RequestType = RequestType::DeleteFile { file_name: (name) };
                                            request::new(request, &email).await
                                        }).await.unwrap();
                                        match result
                                        {
                                            Response::Success => 
                                            {self.remove_item = Some(remove_name);},
                                            Response::Failure(e) => self.error_message = Some(e.clone()),
                                            _ => {},
                                        }
                                    }); 
                                }
                                ui.add_space(10.0);
                            });
                            ui.end_row();
                        }
                        if let Some(name) = &self.remove_item
                        {
                            self.uploadList.retain(|item| item.name != *name);
                            self.remove_item = None;
                        }
                        if self.share_window
                        {
                            Window::new("Share with people")
                            .collapsible(false)
                            .fixed_pos([250.0,250.0])
                            .resizable(false)
                            .hscroll(false)
                            .show(ui.ctx(), |ui| {
                                ui.horizontal(|ui| 
                                {
                                    ui.label(egui::RichText::new("Emails:").color(self.gui_style.color));
                                    ui.text_edit_multiline(&mut self.emails_input);
                                });
        
                                ui.separator();
                                ui.style_mut().visuals.widgets.inactive.weak_bg_fill = self.gui_style.button_color;
                                ui.style_mut().visuals.widgets.inactive.fg_stroke = egui::Stroke{ width: 300.0, color: Color32::WHITE };
                                ui.horizontal(|ui|
                                {
                                    if ui.button("Cancel").clicked() {
                                        self.emails_input.clear();
                                        self.share_window = false;
                                    }
                                    if ui.button("Share").clicked() 
                                    {
                                        let mut emails: Vec<&str> = self.emails_input.split(',').map(|s| s.trim()).collect();
                                        emails.retain(|&s| !s.is_empty());
                                        self.share_with.extend(emails.iter().map(|&s| s.to_string()));
                                        self.emails_input.clear();
                                        let rt= Runtime::new().unwrap();
                                        let email= self.email.clone();
                                        let name = self.selected_file.clone();
                                        let members = self.share_with.clone();
                                        rt.block_on( async
                                        {
                                            let result = tokio::spawn(async move
                                            {
                                                let request:RequestType = RequestType::Share { members: members, filename: name };
                                                request::new(request, &email).await
                                            }).await.unwrap();
                                            match result
                                            {
                                                Response::Success => 
                                                {self.share_with.clear(); self.share_window = false;},
                                                Response::Failure(e) => self.error_message = Some(e.clone()),
                                                _ => {},
                                            }
                                        }); 

                                    }
                                });

                            });
                        }
                        
                    });
                });
            
        // });
        
    
    }

    // Function to show the Shared Files page UI
    fn shared_files_page(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) 
    {
        ui.vertical(|ui|
            {
                ui.add_space(15.0);
                ui.horizontal(|ui| 
                {            
                    ui.add_space(10.0);
                    ui.heading(egui::RichText::new("SHARED WITH ME").color(self.gui_style.color)); // Display heading "My Files"
                });
                ui.add_space(30.0)
            });
            // egui::ScrollArea::auto_sized().show(ui, |ui| 
            // {
                    ui.horizontal(|ui|
                    {
                        ui.add_space(15.0);
                        egui::Grid::new("my_grid").striped(true).num_columns(3).min_col_width(ui.available_width()/6.0).max_col_width(ui.available_width()/2.0).spacing(egui::vec2(30.0, 10.0)).show(ui, |ui| 
                            {
                                ui.label(egui::RichText::new("Name").color(self.gui_style.color).text_style(egui::TextStyle::Body).strong());
                                ui.label(egui::RichText::new("Items").color(self.gui_style.color).text_style(egui::TextStyle::Body).strong());
                                ui.label(egui::RichText::new("Modified").color(self.gui_style.color).text_style(egui::TextStyle::Body).strong());
                                ui.end_row();
                                if self.foldernames.is_empty(){ui.label(egui::RichText::new("No items to show").color(self.gui_style.color));}
                                for row in &self.foldernames
                                {
                                    if ui.hyperlink(row.name.clone()).clicked() 
                                    {
                                        self.selected_folder = row.name.clone();
                                        let foldername = row.name.clone();
                                        self.current_page = Page::FolderDetails;
                                        let rt= Runtime::new().unwrap();
                                        let email= self.email.clone();
                                        rt.block_on( async
                                        {
                                            let result = tokio::spawn(async move
                                            {
                                                let request:RequestType = RequestType::GetFileNames { folder_name: foldername};
                                                request::new(request, &email).await
                                            }).await.unwrap();
                                            println!("{:?}",result);
                                            match result
                                            {
                                                Response::GetFileNameSuccess { filenames } => self.selected_folderlist = filenames,
                                                Response::Failure(e) => self.error_message = Some(e.clone()),
                                                _ => {},
                                            }
                                        }); 
                                    }
                                    ui.label(egui::RichText::new(row.items.to_string()).color(self.gui_style.color));
                                    ui.label(egui::RichText::new(row.updated.clone()).color(self.gui_style.color));
                                    ui.end_row();
                                }
                            });
                    });
                
            // });
    }

    fn display_folder(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui, folder_name:&str)  
    {
        ui.vertical(|ui|
            {
                ui.add_space(15.0);
                ui.horizontal(|ui| 
                {            
                    ui.add_space(10.0);
                    ui.heading(egui::RichText::new(folder_name).color(self.gui_style.color)); // Display heading "My Files"
                });
                ui.add_space(30.0)
            });
            // egui::ScrollArea::auto_sized().show(ui, |ui| 
            // {
                    ui.horizontal(|ui|
                    {
                        ui.add_space(15.0);
                        egui::Grid::new("my_grid").striped(true).num_columns(4).min_col_width(ui.available_width()/7.0).max_col_width(ui.available_width()/3.0).spacing(egui::vec2(30.0, 10.0)).show(ui, |ui| 
                        {
                                ui.label(egui::RichText::new("Title").color(self.gui_style.color).text_style(egui::TextStyle::Body).strong());
                                ui.label(egui::RichText::new("Modified").color(self.gui_style.color).text_style(egui::TextStyle::Body).strong());
                                ui.label(egui::RichText::new("Size").color(self.gui_style.color).text_style(egui::TextStyle::Body).strong()); 
                                ui.end_row();
                                if self.selected_folderlist.is_empty(){ui.label(egui::RichText::new("No items to show").color(self.gui_style.color));}
                                for row in  &self.selected_folderlist
                                {
                                    ui.label(egui::RichText::new(row.name.clone()).color(self.gui_style.color));
                                    ui.label(egui::RichText::new(row.upload_date.clone()).color(self.gui_style.color));
                                    ui.label(egui::RichText::new(row.size.to_string()).color(self.gui_style.color));
                                    ui.horizontal(|ui|
                                    {
                                        ui.style_mut().visuals.widgets.inactive.weak_bg_fill = self.gui_style.button_color;
                                        ui.style_mut().visuals.widgets.inactive.fg_stroke = egui::Stroke{ width: 300.0, color: Color32::WHITE };
                                        if ui.button("Download").clicked() 
                                        {
                                            let name = row.name.clone();
                                            if let Some(path) = rfd::FileDialog::new().pick_folder()
                                            {
                                                let filepath = path.display().to_string();
                                                let rt= Runtime::new().unwrap();
                                                let email= self.email.clone();
                                                rt.block_on( async
                                                {
                                                    let result = tokio::spawn(async move
                                                    {
                                                        let request:RequestType = RequestType::DownloadFile { file_name: (name), file_path: filepath };
                                                        request::new(request, &email).await
                                                    }).await.unwrap();
                                                    match result
                                                    {
                                                        Response::Success => 
                                                        {},
                                                        Response::Failure(e) => self.error_message = Some(e.clone()),
                                                        _ => {},
                                                    }
                                                }); 
                                            }
                                        }
                                        ui.add_space(10.0);
                                    });
                                    ui.end_row();
                                }
                            });
                    });
                
            // });
    }

    fn upload(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui)  
    {
        ui.style_mut() .visuals .widgets .noninteractive .bg_stroke .color = egui::Color32::GRAY;
        ui.vertical(|ui|
        {
            ui.add_space(ui.available_height()/4.0);
            ui.horizontal(|ui|
            {
                ui.add_space((ui.available_width()/2.0)-300.0);
                ui.group(|ui| 
                    {
                        ui.set_max_size(egui::Vec2 { x: 350.0, y: 200.0 });
                        ui.set_min_size(egui::Vec2 { x: 350.0, y: 200.0 });
                        ui.centered_and_justified(|ui|
                            {
                                ui.horizontal(|ui|
                                    {
                                        ui.add_space(50.0);
                                        if ui.button("Select file").clicked()
                                        {
                                            self.error_message = None;
                                            if let Some(path) = rfd::FileDialog::new().pick_file() 
                                            {
                                                self.selected_path = path.display().to_string();
                                            }
                                        }
                                        ui.add_space(5.0);
                                        ui.horizontal_wrapped(|ui|
                                        {
                                            ui.set_max_width(300.0);
                                            let value = self.selected_path.clone();  
                                            if value.len() > 0
                                            {
                                                ui.label(egui::RichText::new(value).color(self.gui_style.color));
                                            }
                                            else 
                                            {
                                                ui.label(egui::RichText::new("No file chosen").color(self.gui_style.color));    
                                            }
                                        });     
                                        ui.add_space(5.0);                                 
                                    });
                                    ui.style_mut().visuals.widgets.inactive.weak_bg_fill = self.gui_style.button_color;
                                    ui.style_mut().visuals.widgets.inactive.fg_stroke = egui::Stroke{ width: 300.0, color: Color32::WHITE };
                                    if ui.button("UPLOAD").clicked()
                                    {
                                        self.error_message = None;
                                        if self.selected_path.is_empty(){
                                            // Show an error message if either the username or password is empty
                                            self.error_message = Some("Select a file".to_string()); // Store error message in a variable
                                        }
                                        else 
                                        {
                                            let rt= Runtime::new().unwrap();
                                            let path= self.selected_path.clone();
                                            let email= self.email.clone();
                                            rt.block_on( async
                                            {
                                                let result = tokio::spawn(async move
                                                {
                                                    let request:RequestType = RequestType::UploadFile { file_path: (path) };
                                                    request::new(request, &email).await
                                                }).await.unwrap();
                                                match result
                                                {
                                                    Response::Success => self.error_message = Some("UPLOAD SUCCESSFUL".to_string()),
                                                    Response::Failure(e) => self.error_message = Some(e.clone()),
                                                    _ => {},
                                                }
                                            }); 
                                        }   
                                    }
                                    if let Some(error_message) = &self.error_message 
                                    {
                                        // Display error message if it exists
                                        ui.add_space(5.0);
                                        ui.label(egui::RichText::new(error_message).color(egui::Color32::RED));
                                    }
                            });
        
                    });
            });

        });                

    }

}

impl App for Gui 
{
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) 
    {

        let frame = egui::containers::Frame 
        {
            shadow: egui::epaint::Shadow{ extrusion: 0.0, color: Color32::TRANSPARENT },
            fill: self.gui_style.side_panel,
            stroke: egui::Stroke::new(0.0, Color32::TRANSPARENT),
            inner_margin: egui::Margin::same(0.0),
            outer_margin: egui::Margin::same(0.0),
            rounding: egui::Rounding::none(),
        };
        egui::CentralPanel::default().frame(frame).show(ctx, |ui| 
        {
            if let Page::Signup = self.current_page {self.signup_page(ctx, ui);}
            else if let Page::Login = self.current_page {self.login_page(ctx, ui);}
            else if let Page::Splash = self.current_page {self.splash(ctx, ui);}
            else{self.home_page(ctx, ui);}
        });
    }
}



