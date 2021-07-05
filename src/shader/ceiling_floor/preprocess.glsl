//Author: Jerrit Gläsker

#version 430
layout(local_size_x = 1, local_size_y = 1) in;

layout(std430, binding = 1) buffer Settings {
	uint resolution_x;
	uint resolution_y;
} settings;

layout(std430, binding = 2) buffer PlayerData {
	vec2 position;
	vec2 direction;
	vec2 camera_plane;
} player;

layout(std430, binding = 3) buffer World {
	uint ceiling_texture_idx;
	uint floor_texture_idx;
	uint geometry_tile_width;
	uint stride;
	uint[] geometry;
} world;

layout(std430, binding = 6) buffer CaFData {
	float data[];
} caf;

void calculate_caf(int y)
{
	float ray_dir_x0 = player.direction.x - player.camera_plane.x;
	float ray_dir_y0 = player.direction.y - player.camera_plane.y;
	float ray_dir_x1 = player.direction.x + player.camera_plane.x;
	float ray_dir_y1 = player.direction.y + player.camera_plane.y;

	int p = y - int(settings.resolution_y) / 2;

	float pos_z = 0.5 * float(settings.resolution_y);

	float row_dist = pos_z / p;

	float floor_step_x = row_dist * (ray_dir_x1 - ray_dir_x0) / settings.resolution_x;
	float floor_step_y = row_dist * (ray_dir_y1 - ray_dir_y0) / settings.resolution_x;

	float floor_x = player.position.x + row_dist * ray_dir_x0;
	float floor_y = player.position.y + row_dist * ray_dir_y0;

	caf.data[y * 4 + 0] = floor_x;
	caf.data[y * 4 + 1] = floor_y;
	caf.data[y * 4 + 2] = floor_step_x;
	caf.data[y * 4 + 3] = floor_step_y;
}

void main()
{
	ivec2 iCoords = ivec2(gl_GlobalInvocationID.xy);

	calculate_caf(iCoords.y);
}
