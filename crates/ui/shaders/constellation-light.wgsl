// Translucent folds and a perimeter highlight. Position and size share pixel units.
fn light_packet(along: f32, center: f32, head: f32, tail: f32) -> f32 {
    let offset = fract(along - center + 0.5) - 0.5;
    return 1.0 - smoothstep(0.006, select(tail, head, offset > 0.0), abs(offset));
}

// Give neighboring sides the same coordinate where their inner glows meet.
fn edge_coordinate(p: vec2<f32>, size: vec2<f32>, depth: f32) -> f32 {
    let far = size - p;
    let straight = max(size - vec2<f32>(2.0 * depth), vec2<f32>(1.0));
    let pos = clamp((p - vec2<f32>(depth)) / straight, vec2<f32>(0.0), vec2<f32>(1.0));
    if depth == p.y {
        return pos.x * size.x;
    }
    if depth == far.x {
        return size.x + pos.y * size.y;
    }
    if depth == far.y {
        return size.x + size.y + (1.0 - pos.x) * size.x;
    }
    return 2.0 * size.x + size.y + (1.0 - pos.y) * size.y;
}

fn flowing_rim(input: ShaderInput) -> vec4<f32> {
    let size = input.size;
    let p = input.uv * size;
    let time = input.time;
    let bass = smoothstep(0.04, 0.75, input.params[0].y);
    let highs = smoothstep(0.025, 0.58, input.params[0].z);
    let far = size - p;
    let depth = min(min(p.x, p.y), min(far.x, far.y));
    let unit = max(min(size.x, size.y) / 1080.0, 0.5);
    let inset = depth / unit;
    if inset >= 180.0 {
        return vec4<f32>(0.0);
    }
    let perimeter = max(2.0 * (size.x + size.y), 1.0);
    let along = edge_coordinate(p, size, depth) / perimeter;
    let cycle = time * 0.78 + 0.11 * sin(time * 1.36) + input.params[0].x * 0.10;
    let primary = light_packet(along, cycle, 0.14 + bass * 0.025, 0.24);
    let secondary = light_packet(along, cycle + 0.46 + 0.055 * sin(time * 1.08), 0.11, 0.21 + bass * 0.025);
    let echo = light_packet(along, cycle + 0.68, 0.10, 0.15) * 0.36;
    let change = input.params[0].w;
    let wash = sin(change * 3.14159265) * light_packet(along, cycle + change * 0.38, 0.07, 0.21);
    let lit = max(max(primary, secondary), echo);

    // A moving narrow highlight sits on top of wider colored light. Noise and
    // waves change the depth of the glow without shifting the edge itself.
    let angle = along * 6.2831853;
    let grain = gradient_noise(vec2<f32>(cos(angle), sin(angle)) * 7.0 + vec2<f32>(-time * 0.31, time * 0.47));
    let folds = 0.52 + 0.48 * sin(angle * 9.0 - time * 3.6 + grain * 2.7);
    let reach = min(178.0, (38.0 + (67.0 + 51.0 * bass) * lit) * (0.79 + folds * 0.32 + grain * 0.26));
    let halo = pow(1.0 - smoothstep(0.0, reach, inset), 1.65);
    let spill = 1.0 - smoothstep(0.0, 32.0 + (14.0 + bass * 17.0) * lit, inset);
    let core = 1.0 - smoothstep(0.45, 4.2 + 2.0 * highs, inset);
    let ripples = smoothstep(0.72, 0.99, sin(angle * 17.0 - time * 5.1 + grain * 3.2));
    let shimmer = highs * ripples * lit;
    let alpha = core * (0.09 + 0.56 * lit + 0.26 * shimmer + 0.15 * wash)
        + spill * (0.07 + 0.31 * lit + 0.14 * shimmer)
        + halo * (0.055 + (0.34 + 0.26 * bass) * lit + 0.24 * wash);

    let hue = mix(input.colors[1].rgb, input.colors[2].rgb,
        smoothstep(0.04, 0.62, secondary - primary * 0.48 + echo * 0.28));
    let color = mix(hue, vec3<f32>(1.0), core * min(1.0, 0.31 + 0.38 * lit + 0.25 * shimmer + 0.15 * wash));
    return vec4<f32>(color, min(alpha, 0.98));
}

fn shade(input: ShaderInput) -> vec4<f32> {
    let t = input.time;
    let energy = input.params[0].y;
    let uv = input.uv;
    let rim = flowing_rim(input);

    let warped_x = uv.x + 0.09 * sin(uv.y * 5.5 + t * 0.74);
    let folds = sin(warped_x * 27.0 + uv.y * 10.0 + 2.2 * sin(uv.y * 5.0 - t * 0.69) - t * 1.6);
    let creases = smoothstep(0.77, 0.99, folds);
    let canopy = 1.0 - smoothstep(0.025, 0.41 + energy * 0.05, uv.y);
    let veil = 0.025 + 0.12 * canopy;
    let wave = uv.y - (0.72 + 0.09 * sin(uv.x * 8.0 + t) + 0.03 * sin(uv.x * 19.0 - t * 0.63));
    let wisp = exp(-abs(wave) * 68.0) * (0.04 + 0.10 * smoothstep(-0.5, 0.9, sin(uv.x * 11.0 - t)));
    let fabric = (creases * veil + wisp) * (0.78 + energy * 0.22);
    let pearl = mix(vec3<f32>(1.0), input.colors[0].rgb, 0.18);
    let background_alpha = fabric * (1.0 - rim.a);
    let alpha = rim.a + background_alpha;
    let color = (rim.rgb * rim.a + pearl * background_alpha) / max(alpha, 0.0001);
    return vec4<f32>(color, alpha);
}
