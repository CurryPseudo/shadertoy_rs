use std::borrow::Cow;

#[cfg(not(target_arch = "wasm32"))]
pub fn convert_shader(source: &str, stage: shaderc::ShaderKind) -> crate::app::Result<Vec<u32>> {
    use anyhow::anyhow;
    // use shaderc to compile the shader
    let compiler = shaderc::Compiler::new().ok_or_else(|| anyhow!("Failed to create compiler"))?;
    // Compile the shader and disable most warnings
    let mut compile_options = shaderc::CompileOptions::new()
        .ok_or_else(|| anyhow!("Failed to create compile options"))?;
    compile_options.set_suppress_warnings();
    let binary_result = compiler.compile_into_spirv(
        source,
        stage,
        "shader.glsl",
        "main",
        Some(&compile_options),
    )?;
    Ok(binary_result.as_binary().into())
}
#[cfg(not(target_arch = "wasm32"))]
macro_rules! load_shader {
    ($path:literal) => {{
        let path = format!("src/app/{}", $path);
        std::fs::read_to_string(&std::path::Path::new(&path))?
    }};
}

pub fn load_vertex_shader() -> crate::app::Result<Cow<'static, [u32]>> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        Ok(convert_shader(&load_shader!("shader.vert"), shaderc::ShaderKind::Vertex)?.into())
    }
    #[cfg(target_arch = "wasm32")]
    {
        let bytes = include_bytes!("shader.vert.spv");
        Ok(Cow::from(bytemuck::cast_slice(bytes)))
    }
}
pub fn load_fragment_shader(content: &str) -> crate::app::Result<Cow<'static, [u32]>> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let template = load_shader!("shader.frag");
        let map = [("content".to_string(), content.to_string())]
            .into_iter()
            .collect::<std::collections::HashMap<String, String>>();
        convert_shader(
            &strfmt::strfmt(template.as_str(), &map)?,
            shaderc::ShaderKind::Fragment,
        )
        .map(Cow::from)
    }
    #[cfg(target_arch = "wasm32")]
    {
        let _ = content;
        let bytes = include_bytes!("shader.frag.spv");
        Ok(Cow::from(bytemuck::cast_slice(bytes)))
    }
}

/*
#[test]
fn shader_error() {
    let source = include_str!("test/error.frag");
    let result = convert_shader(source, naga::ShaderStage::Fragment);
    if let Err(err) = result {
        assert_eq!(err.to_string(), include_str!("test/error.frag.error"));
    } else {
        panic!("Expected an error, but got: {:?}", result);
    }
}
*/
