#[cfg(target_os = "windows")]
pub async fn list() {
  println!("{}", list_windows::list(1));
}

#[cfg(target_os = "linux")]
pub async fn list() {
  println!("{}", list_linux::list(1, 2));
}

#[cfg(target_os = "darwin")]
pub async fn list() {
  println!("{}", list_darwin::list(1, 2, 3));
}
