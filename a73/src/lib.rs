#[no_mangle]
pub extern "C" fn scene_new(
    width: u32,
    height: u32,
    fov: f64,
    shadow_bias: f64,
    max_recursion_depth: u32,
) -> *mut Scene {
    let scene = Box::new(Scene {
        width: width,
        height: height,
        fov: fov,

        elements: vec![],
        lights: vec![],

        shadow_bias: shadow_bias,
        max_recursion_depth: max_recursion_depth,
    });
    Box::into_raw(scene)
}
