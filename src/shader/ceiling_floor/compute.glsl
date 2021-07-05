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

layout(binding = 4) uniform sampler2DArray texAtlas;

layout(std430, binding = 6) buffer CaFData {
	float data[];
} caf;

float line_to_lod(int y)
{
	int p = y - int(settings.resolution_y) / 2;
	float pos_z = 0.5 * float(settings.resolution_y);
	float row_dist = pos_z / p;

	float lod = 0.0;

	for (float dist = row_dist; dist >= 0 && lod < 5.0; dist -= 1)
		lod += 0.1;
	return lod;
}

vec4 get_atlas_color(int textureIdx, int x, int y, float lod)
{
	float x_n = float(x) / float(world.geometry_tile_width);
	float y_n = float(y) / float(world.geometry_tile_width);

	float x_center_offset = 1.0 / (float(world.geometry_tile_width) * 2.0);
	float y_center_offset = 1.0 / (float(world.geometry_tile_width) * 2.0);

	return textureLod(texAtlas, vec3(x_n + x_center_offset, y_n + y_center_offset, textureIdx), lod);
}

void compute(ivec2 iCoords)
{
	float floor_x = caf.data[iCoords.y * 4 + 0];
	float floor_y = caf.data[iCoords.y * 4 + 1];
	float floor_step_x = caf.data[iCoords.y * 4 + 2];
	float floor_step_y = caf.data[iCoords.y * 4 + 3];

	floor_x += iCoords.x * floor_step_x;
	floor_y += iCoords.x * floor_step_y;

	int cell_x = int(floor_x);
	int cell_y = int(floor_y);

	int tx = int(world.geometry_tile_width * (floor_x - cell_x)) & int(world.geometry_tile_width - 1);
	int ty = int(world.geometry_tile_width * (floor_y - cell_y)) & int(world.geometry_tile_width - 1);

	int floor_tex_idx = int(world.floor_texture_idx);
	int ceiling_tex_idx = int(world.ceiling_texture_idx);

	//floor
	vec4 color = get_atlas_color(floor_tex_idx - 1, tx, ty, line_to_lod(iCoords.y));

	imageStore(img, iCoords, color);
	//ceiling
	color = get_atlas_color(ceiling_tex_idx - 1, tx, ty, line_to_lod(iCoords.y));
	ivec2 pos = ivec2(iCoords.x, settings.resolution_y - iCoords.y - 1);

	imageStore(img, pos, color);
}

void main()
{
	ivec2 iCoords = ivec2(gl_GlobalInvocationID.xy);

	compute(iCoords);
}
