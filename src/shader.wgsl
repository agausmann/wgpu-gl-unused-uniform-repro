[[block]]
struct Viewport {
    proj: mat4x4<f32>;
};

[[group(0), binding(0)]]
var<uniform> viewport: Viewport;

[[stage(vertex)]]
fn main([[location(0)]] position: vec2<f32>) -> [[builtin(position)]] vec4<f32> {
    return viewport.proj * vec4<f32>(position, 0.0, 1.0);
}

[[stage(fragment)]]
fn main() -> [[location(0)]] vec4<f32> {
    // Uncomment this line to fix the panic
    //var _ = viewport.proj;

    return vec4<f32>(0.0, 0.0, 0.0, 1.0);
}
