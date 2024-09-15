
#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]


mod color;
mod web;
mod standards;
mod omni_modules;
mod omni_main;
use omni_modules::omni_modules::*;


fn main() {


  #[tauri::command]
  fn search(search: String)->String{
    let mod_vec: Vec<Module> = omni_main::omni_main::get_modules();
    let sd = scan_mods(&mod_vec, search.clone());
    let serializedn = serde_json::to_string(&sd).unwrap();
    println!("{}", serializedn);
    serializedn
  }
  #[tauri::command]
  fn result(search: String, module: String, uid: String)->String {
    let mod_vec: Vec<Module> = omni_main::omni_main::get_modules();
    let test_act = get_action_from_uid(&mod_vec, &module, &uid).unwrap();
    test_act.result(search)
  }


  #[tauri::command]
  fn en_acr(window: tauri::Window){
    apply_acrylic(&window);
  }


  use tauri::Manager;
  use window_vibrancy::*;

  let app = tauri::Builder::default()
    .setup(|app| {
      let window = app.get_window("main").unwrap();
      #[cfg(target_os = "windows")]
      apply_blur(&window).unwrap();


      #[cfg(target_os = "macos")]
      {
        use tauri_plugin_vibrancy::MacOSVibrancy;
        window.apply_vibrancy(MacOSVibrancy::AppearanceBased);
      }
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![search, result, en_acr])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
  
}
