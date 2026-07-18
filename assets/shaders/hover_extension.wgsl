#import bevy_pbr::{
    mesh_view_bindings::{globals, view},
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::alpha_discard,
    utils::coords_to_viewport_uv,
}

#ifdef PREPASS_PIPELINE
#import bevy_pbr::{
    prepass_io::{VertexOutput, FragmentOutput},
    pbr_deferred_functions::deferred_output,
}
#else
#import bevy_pbr::{
    forward_io::{VertexOutput, FragmentOutput},
    pbr_functions::{apply_pbr_lighting, main_pass_post_lighting_processing},
}
#endif


struct HoverExtension {
    is_active: u32,
    trans_start: f32,
}

@group(#{MATERIAL_BIND_GROUP}) @binding(100)
var<uniform> hover_extension: HoverExtension;

@fragment
fn fragment(
    in: VertexOutput,
    @builtin(front_facing) is_front: bool,
) -> FragmentOutput {
    var pbr_input = pbr_input_from_standard_material(in, is_front);

    pbr_input.material.base_color = alpha_discard(pbr_input.material, pbr_input.material.base_color);

#ifdef PREPASS_PIPELINE
    let out = deferred_output(in, pbr_input);
#else
    var out: FragmentOutput;

    let elapsed = globals.time - hover_extension.trans_start;
    let TRANS_TIME = 0.150;

    if elapsed < TRANS_TIME {
      let viewport_uv = coords_to_viewport_uv(in.position.xy, view.viewport);
      let hovered = pbr_input.material.base_color * 0.3 + main(viewport_uv);

      let light = apply_pbr_lighting(pbr_input);
      let standard_material = main_pass_post_lighting_processing(pbr_input, light);

      let t = elapsed / TRANS_TIME;

      if hover_extension.is_active == 1 {
        out.color = hovered * t + standard_material * (0.5 - t / 2);
      } else {
        out.color = hovered * (0.5 - t / 2) + standard_material * t;
      }
    } else if hover_extension.is_active == 1 {
      let viewport_uv = coords_to_viewport_uv(in.position.xy, view.viewport);
      out.color = pbr_input.material.base_color * 0.3 + main(viewport_uv);
    } else {
      out.color = apply_pbr_lighting(pbr_input);
      out.color = main_pass_post_lighting_processing(pbr_input, out.color);
    }
#endif

    return out;
}

fn main(uv: vec2<f32>) -> vec4<f32> {
    let color = vec3<f32>(0., 0., 1.);

    let time = globals.time % 5 / 5;

    let a = time * 3.14159 * 2;

    let out = color * calc_point(time, a, uv * 10);
    return vec4<f32>(out, 1.);
}

fn gray(c: vec3<f32>) -> f32 {
    return (c.x + c.y + c.z) / 3;
}

fn calc_point(t: f32, a: f32, uv: vec2<f32>) -> f32 {
    return tanh(
        pow(abs(sin(uv.y * 3.14159 + a)) * 7, -1)
        + pow(abs(cos(uv.x * 3.14159 + a + cos(a))) * 7, -1)
        // +
        // max(0, 1 - sqrt(pow((uv.x - t) * 3, 2) + pow((uv.y - t) * 3, 2)))
        // +
        // max(0, 1 - sqrt(pow((uv.x - 0.5) * 3, 2) + pow((uv.y - t) * 3, 2)))
        // +
        // max(0, 1 - sqrt(pow((uv.x - t) * 3, 2) + pow((uv.y - 0.5) * 3, 2)))
        // +
        // max(0, 1 - sqrt(pow((t - uv.x) * 3, 3) + pow((uv.y - 0.5) * 3, 3)))
    ) * 0.75;
}
