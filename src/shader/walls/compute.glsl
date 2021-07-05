//Author: Jerrit Gläsker

#version 430
layout(local_size_x = 1, local_size_y = 1) in;
layout(rgba32f, binding = 0) uniform image2D img;

layout(std430, binding = 1) buffer Settings {
	uint resolution_x;
	uint resolution_y;
} settings;

layout(std430, binding = 3) buffer World {
	uint ceiling_texture_idx;
	uint floor_texture_idx;
	uint geometry_tile_width;
	uint stride;
	uint[] geometry;
} world;

layout(binding = 4) uniform sampler2DArray tex_atlas;

layout(std430, binding = 5) buffer SliceData {
	int data[];
} slice;

float line_height_to_lod(int line_height)
{
	double perp_wall_dist = double(settings.resolution_x) / double(line_height);

	float lod = 0.0;

	for (double dist = perp_wall_dist; dist >= 0 && lod < 10.0; dist -= 1)
		lod += 0.025;
	return lod;
}

vec4 get_atlas_color(int texture_idx, int x, int y, float lod)
{
	float x_n = float(x) / float(world.geometry_tile_width);
	float y_n = float(y) / float(world.geometry_tile_width);

	float x_center_offset = 1.0 / (float(world.geometry_tile_width) * 2.0);
	float y_center_offset = 1.0 / (float(world.geometry_tile_width) * 2.0);

	return textureLod(tex_atlas, vec3(x_n + x_center_offset, y_n + y_center_offset, texture_idx), lod);
}

vec4 process_slice(ivec2 iCoords)
{
	int x = iCoords.x;

	int line_height = slice.data[x * 3 + 0];
	int tex_idx = slice.data[x * 3 + 1];
	int texX = slice.data[x * 3 + 2];

	int draw_start = int(-line_height / 2.0 + int(settings.resolution_y) / 2.0);

	if (draw_start < 0) draw_start = 0;
	int draw_end = int(line_height / 2.0 + int(settings.resolution_y) / 2.0);

	if (draw_end >= settings.resolution_y) draw_end = int(settings.resolution_y - 1);

	int y = iCoords.y;
	double tex_step = 1.0 * int(world.geometry_tile_width) / line_height;
	double tex_pos = (y - settings.resolution_y / 2 + line_height / 2) * tex_step;

	if (y < draw_start || y >= draw_end) {
		return imageLoad(img, iCoords);
	} else {
		int texY = int(tex_pos) & (int(world.geometry_tile_width) - 1);
		return get_atlas_color(int(tex_idx), texX, texY, line_height_to_lod(line_height));
	}
}

void main()
{
	ivec2 iCoords = ivec2(gl_GlobalInvocationID.xy);

	vec4 color = process_slice(iCoords);

	imageStore(img, iCoords, color);
}
