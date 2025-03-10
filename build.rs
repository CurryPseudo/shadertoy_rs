fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    {
        // use shaderc to compile the shader
        let compiler = shaderc::Compiler::new().unwrap();
        // Compile the shader and disable most warnings
        let mut compile_options = shaderc::CompileOptions::new().unwrap();
        compile_options.set_suppress_warnings();
        let vertex_shader = include_str!("src/app/shader.vert");
        let binary_result = compiler
            .compile_into_spirv(
                vertex_shader,
                shaderc::ShaderKind::Vertex,
                "shader.vert",
                "main",
                Some(&compile_options),
            )
            .unwrap();
        std::fs::write("src/app/shader.vert.spv", binary_result.as_binary_u8()).unwrap();
        let fragment_shader_template = include_str!("src/app/shader.frag");
        let content = include_str!("src/app/default.glsl");
        let map = std::iter::once(("content".to_string(), content.to_string()))
            .collect::<std::collections::HashMap<String, String>>();
        let fragment_shader = strfmt::strfmt(fragment_shader_template, &map).unwrap();
        let binary_result = compiler
            .compile_into_spirv(
                &fragment_shader,
                shaderc::ShaderKind::Fragment,
                "shader.frag",
                "main",
                Some(&compile_options),
            )
            .unwrap();
        std::fs::write("src/app/shader.frag.spv", binary_result.as_binary_u8()).unwrap();
    }
}
