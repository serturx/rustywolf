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

void main()
{
	ivec3 iCoords = ivec3(gl_GlobalInvocationID.xyz);

	double sprite_x = sprites.list[iCoords.z].x_pos - player.position.x;
	double sprite_y = sprites.list[iCoords.z].y_pos - player.position.y;

	double inv_det = 1.0 / (player.camera_plane.x * player.direction.y - player.direction.x * player.camera_plane.y);

	double transform_x = inv_det * (player.direction.y * sprite_x - player.direction.x * sprite_y);
	double transform_y = inv_det * (-player.camera_plane.y * sprite_x + player.camera_plane.x * sprite_y);

	int sprite_screen_x = int((settings.resolution_x / 2) * (1 + transform_x / transform_y));

	int sprite_width = abs(int(settings.resolution_y / transform_y));
	int sprite_height = abs(int(settings.resolution_y / transform_y));

	int draw_start_y = -sprite_height / 2 + settings.resolution_y / 2;

	if (draw_start_y < 0)
		draw_start_y = 0;

	int draw_end_y = sprite_height / 2 + settings.resolution_y / 2;

	if (draw_end_y >= settings.resolution_y)
		draw_end_y = settings.resolution_y - 1;

	int draw_start_x = -sprite_width / 2 + sprite_screen_x;

	if (draw_start_x < 0)
		draw_start_x = 0;

	int draw_end_x = sprite_width / 2 + sprite_screen_x;

	if (draw_end_x >= settings.resolution_x)
		draw_end_x = settings.resolution_x - 1;

	sprite_preprocess.results[iCoords.z] = Sprite_Preprocess_Result(
		sprite_width,
		sprite_height,

		draw_start_y,
		draw_end_y,
		draw_start_x,
		draw_end_x,

		sprite_screen_x,
		transform_y
		);
}
