// A simple color gradient shader
void mainImage(out vec4 fragColor, in vec2 fragCoord) {
    // Normalized coordinates [0, 1]
    vec2 uv = fragCoord / iResolution.xy;
    
    // Create gradient color
    vec3 color = vec3(uv.x, uv.y, sin(iTime) * 0.5 + 0.5);
    
    // Output color
    fragColor = vec4(color, 1.0);
} 