// This shader is inspired by Start Nest by Pablo Roman Andrioli:
// https://www.shadertoy.com/view/XlfGRj

// The time since startup data is in the globals binding which is part of the mesh_view_bindings import
#import bevy_sprite::mesh2d_view_bindings

struct WeatherMaterial {
    time: f32,
};

@group(1) @binding(0)
var<uniform> weather: WeatherMaterial;

const LAYERS = 8;
const DEPTH = 0.5;
const WIDTH = 0.8;
const SPEED = 0.3;

@fragment
fn fragment(
    #import bevy_sprite::mesh2d_vertex_output
) -> @location(0) vec4<f32> {
    let p: mat3x3<f32> = mat3x3<f32>(13.323122, 23.5112, 21.71123, 21.1212, 28.7312, 11.9312, 21.8112, 14.7212, 61.3934);
	var acc: vec3<f32> = vec3<f32>(0.);
	let dof: f32 = 5. * sin(weather.time * 0.1);
	for (var i=0;i<LAYERS;i++) {
		let fi: f32 = f32(i);
		var q: vec2<f32> = uv * (1. + fi * DEPTH);
		q += vec2(q.y*(WIDTH*(fi*7.238917%1.)-WIDTH*.5),SPEED*weather.time/(1.+fi*DEPTH*.03));
		let n: vec3<f32> = vec3<f32>(floor(q), 31.189 + fi);
		let m: vec3<f32> = floor(n) * 0.00001 + fract(n);
		let mp: vec3<f32> = (31415.9 + m) / fract(p * m);
		let r: vec3<f32> = fract(mp);
		var s: vec2<f32> = abs(((q) % (1.)) - 0.5 + 0.9 * r.xy - 0.45);
		s += .01*abs(2.*fract(10.*q.yx));
		let d: f32 = 0.6 * max(s.x - s.y, s.x + s.y) + max(s.x, s.y) - 0.01;
		let edge: f32 = 0.005 + 0.05 * min(0.5 * abs(fi - 5. - dof), 1.);
		acc += vec3(smoothstep(edge,-edge,d)*(r.x/(1.+.02*fi*DEPTH)));
	}
	return vec4(vec3(acc),1.0);
}