#version 430
layout(local_size_x = 1, local_size_y = 1) in;
layout(rgba32f, binding = 0) uniform image2D img;

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

layout(binding = 4) uniform sampler2DArray tex_atlas;

layout(std430, binding = 7) buffer zData {
	double data[];
} z_buffer;

struct Sprite {
	float	x_pos;
	float	y_pos;
	float	x_dir;
	float	y_dir;

	int	texture_base_index;
	int	animation_count;
	int	view_angle_count;
	int	tile_width;
	int	tile_height;

	int	animation_index;
};

layout(std430, binding = 8) buffer SpriteBuffer {
	Sprite list[];
} sprites;

struct Sprite_Preprocess_Result {
	int	sprite_width;
	int	sprite_height;

	int	draw_start_y;
	int	draw_end_y;
	int	draw_start_x;
	int	draw_end_x;

	int	sprite_screen_x;
	double	transform_y;
};

//Should have the same length as SpriteBuffer
layout(std430, binding = 9) buffer Sprite_Preprocess_Results {
	Sprite_Preprocess_Result results[];
} sprite_preprocess;

uniform uint sprite_idx;

#define M_PI 3.141592654

float line_height_to_lod(int line_height)
{
	double perp_wall_dist = double(settings.resolution_x) / double(line_height);

	float lod = 0.0;

	for (double dist = perp_wall_dist - 10; dist >= 0 && lod < 10.0; dist -= 2)
		lod += 0.2;
	return lod;
}

vec4 get_atlas_color(Sprite sprite, int view_angle_idx, int x, int y, float lod)
{
	float x_n = float(x) / float(sprite.tile_width);
	float y_n = float(y) / float(sprite.tile_height);

	float x_center_offset = 1.0 / (float(sprite.tile_width) * 2.0);
	float y_center_offset = 1.0 / (float(sprite.tile_height) * 2.0);

	return textureLod(
		tex_atlas,
		vec3(
			x_n + x_center_offset,
			y_n + y_center_offset,
			sprite.texture_base_index + (sprite.animation_index + view_angle_idx * (sprite.animation_count + 1))),
		lod);
}

float map(float value, float in_min, float in_max, float out_min, float out_max)
{
	return out_min + (out_max - out_min) * (value - in_min) / (in_max - in_min);
}

void main()
{
	ivec2 iCoords = ivec2(gl_GlobalInvocationID.xy);

	Sprite sprite = sprites.list[sprite_idx];

	float dx = player.position.x - sprite.x_pos;
	float dy = player.position.y - sprite.y_pos;

	float angle_to_player = atan(dy, dx) - atan(sprite.y_dir, sprite.x_dir) - (M_PI / 4);

	//move angle_to_player to the range [0,2PI)
	if (angle_to_player < 0)
		angle_to_player += 2 * M_PI;

	int view_angle_idx = int(map(angle_to_player, 2 * M_PI, 0, 0, sprite.view_angle_count));

	Sprite_Preprocess_Result preprocess = sprite_preprocess.results[sprite_idx];

	iCoords += ivec2(preprocess.draw_start_x, preprocess.draw_start_y);

	if (preprocess.transform_y < z_buffer.data[iCoords.x]) {
		int d = iCoords.y * 256 - settings.resolution_y * 128 + preprocess.sprite_height * 128;

		int tex_x = int(256 * (iCoords.x - (-preprocess.sprite_width / 2 + preprocess.sprite_screen_x)) * sprite.tile_width / preprocess.sprite_width) / 256;
		int tex_y = int(((d * sprite.tile_height) / preprocess.sprite_height) / 256);

		vec4 color = get_atlas_color(sprite, view_angle_idx, tex_x, tex_y, line_height_to_lod(preprocess.draw_end_y - preprocess.draw_start_y));
		vec4 base = imageLoad(img, iCoords);

		color = (color * color.a) + (base * (1 - color.a));

		imageStore(img, iCoords, color);
	}
}
