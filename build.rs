#[cfg(target_os = "windows")]
fn main() {
  let src = ["src-drivelist/windows/list.cpp"];

  let mut builder = cc::Build::new();

  let build = builder
    .cpp(true)
    .flag_if_supported()
    .files(src.iter());

  build.compile("drivelist");
}
