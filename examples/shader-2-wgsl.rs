use shaderc;
use std::path::Path;
use naga::front;
use naga::back;

fn main() {
  let mut compiler = shaderc::Compiler::new().unwrap();
  let  options = shaderc::CompileOptions::new().unwrap();
  let shaders = [
    "src/shaders/main.frag",
    "src/shaders/main.vert"
  ];

  for &shader in shaders.iter() {
    let source = std::fs::read_to_string(shader).unwrap();
    let spv = compiler.compile_into_spirv(
      &source, get_shader_kind(shader),
      shader, "main", Some(&options),
    ).unwrap();
    let naga_module =
      front::spv::parse_u8_slice(spv.as_binary_u8(), &front::spv::Options {
        ..Default::default()
      });
    let naga_module = match naga_module {
      Ok(nm) => nm,
      Err(e) =>{
        panic!("shader compile error!{:?}", e)
      }
    };

    println!("{:?}", naga_module);
  }
}

fn get_shader_kind(path: &str) -> shaderc::ShaderKind {
  use shaderc::ShaderKind;
  let path: &Path = path.as_ref();
  let ext = path.extension().unwrap().to_str().unwrap();
  match ext {
    "vert" => ShaderKind::Vertex,
    "frag" => ShaderKind::Fragment,
    "comp" => ShaderKind::Compute,
    "tessc" => ShaderKind::TessControl,
    "tess" => ShaderKind::TessControl,
    "tesse" => ShaderKind::TessEvaluation,
    _ => ShaderKind::InferFromSource
  }
}