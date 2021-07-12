//Author: Jerrit Gläsker

#version 430
layout(local_size_x = 1, local_size_y = 1) in;

layout(std430, binding = 1) buffer Settings {
	int resolution_x;
	int resolution_y;
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

layout(std430, binding = 5) buffer SliceData {
	int data[];
} slice;

layout(std430, binding = 7) buffer zData {
	double data[];
} z_buffer;

uint get_world_value(int x, int y)
{
	return world.geometry[x + y * world.stride];
}

void precompute_slice(int x)
{
	double camera_x = 2 * x / double(settings.resolution_x) - 1;
	double ray_dir_x = player.direction.x + player.camera_plane.x * camera_x;
	double ray_dir_y = player.direction.y + player.camera_plane.y * camera_x;

	int map_x = int(player.position.x);
	int map_y = int(player.position.y);

	double side_dist_x;
	double side_dist_y;

	double delta_dist_x = abs(1 / ray_dir_x);
	double delta_dist_y = abs(1 / ray_dir_y);
	double perp_wall_dist;

	int step_x;
	int step_y;

	int hit = 0;
	int side;

	if (ray_dir_x < 0) {
		step_x = -1;
		side_dist_x = (player.position.x - map_x) * delta_dist_x;
	} else {
		step_x = 1;
		side_dist_x = (map_x + 1.0 - player.position.x) * delta_dist_x;
	}
	if (ray_dir_y < 0) {
		step_y = -1;
		side_dist_y = (player.position.y - map_y) * delta_dist_y;
	} else {
		step_y = 1;
		side_dist_y = (map_y + 1.0 - player.position.y) * delta_dist_y;
	}

	//Maps which are not fully closed will stall the gpu, since the loop below will be infinite
	//TODO implement protection/limit in case of a not fully closed map
	while (hit == 0) {
		if (side_dist_x < side_dist_y) {
			side_dist_x += delta_dist_x;
			map_x += step_x;
			side = 0;
		} else {
			side_dist_y += delta_dist_y;
			map_y += step_y;
			side = 1;
		}
		if (get_world_value(map_x, map_y) > 0) hit = 1;
	}

	if (side == 0) perp_wall_dist = (map_x - player.position.x + (1 - step_x) / 2) / ray_dir_x;
	else perp_wall_dist = (map_y - player.position.y + (1 - step_y) / 2) / ray_dir_y;

	int line_height = int(settings.resolution_y / perp_wall_dist);

	int tex_idx = int(get_world_value(map_x, map_y) - 1);

	double wall_x;

	if (side == 0) wall_x = player.position.y + perp_wall_dist * ray_dir_y;
	else wall_x = player.position.x + perp_wall_dist * ray_dir_x;
	wall_x -= floor(wall_x);

	int texX = int(wall_x * double(world.geometry_tile_width));

	if (side == 0 && ray_dir_x > 0) texX = int(world.geometry_tile_width) - texX - 1;
	if (side == 1 && ray_dir_y < 0) texX = int(world.geometry_tile_width) - texX - 1;

	//Store calculations
	slice.data[x * 3 + 0] = line_height;
	slice.data[x * 3 + 1] = tex_idx;
	slice.data[x * 3 + 2] = texX;

	z_buffer.data[x] = perp_wall_dist;
}

void main()
{
	ivec2 iCoords = ivec2(gl_GlobalInvocationID.xy);

	precompute_slice(iCoords.x);
}
