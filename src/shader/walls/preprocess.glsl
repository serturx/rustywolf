//Author: Jerrit Gläsker

#version 430
layout(local_size_x = 1, local_size_y = 1) in;

layout(std430, binding = 1) buffer Settings {
	int ResolutionX;
	int ResolutionY;
} settings;

layout(std430, binding = 2) buffer PlayerData {
	vec2 position;
	vec2 direction;
	vec2 camPlane;
} player;

layout(std430, binding = 3) buffer World {
	uint ceilingTexIdx;
	uint floorTexIdx;
	uint geometryTileWidth;
	uint stride;
	uint[] geometry;
} world;

layout(std430, binding = 5) buffer SliceData {
	int data[];
} sliceData;

uint GetWorldValue(int x, int y)
{
	return world.geometry[x + y * world.stride];
}

void PrecomputeSlice(int x)
{
	double cameraX = 2 * x / double(settings.ResolutionX) - 1;
	double rayDirX = player.direction.x + player.camPlane.x * cameraX;
	double rayDirY = player.direction.y + player.camPlane.y * cameraX;

	int mapX = int(player.position.x);
	int mapY = int(player.position.y);

	double sideDistX;
	double sideDistY;

	double deltaDistX = abs(1 / rayDirX);
	double deltaDistY = abs(1 / rayDirY);
	double perpWallDist;

	int stepX;
	int stepY;

	int hit = 0;
	int side;

	if (rayDirX < 0) {
		stepX = -1;
		sideDistX = (player.position.x - mapX) * deltaDistX;
	} else {
		stepX = 1;
		sideDistX = (mapX + 1.0 - player.position.x) * deltaDistX;
	}
	if (rayDirY < 0) {
		stepY = -1;
		sideDistY = (player.position.y - mapY) * deltaDistY;
	} else {
		stepY = 1;
		sideDistY = (mapY + 1.0 - player.position.y) * deltaDistY;
	}

	//Maps which are not fully closed will stall the gpu, since the loop below will be infinite
	//TODO implement protection/limit in case of a not fully closed map
	while (hit == 0) {
		if (sideDistX < sideDistY) {
			sideDistX += deltaDistX;
			mapX += stepX;
			side = 0;
		} else {
			sideDistY += deltaDistY;
			mapY += stepY;
			side = 1;
		}
		if (GetWorldValue(mapX, mapY) > 0) hit = 1;
	}

	if (side == 0) perpWallDist = (mapX - player.position.x + (1 - stepX) / 2) / rayDirX;
	else perpWallDist = (mapY - player.position.y + (1 - stepY) / 2) / rayDirY;

	int lineHeight = int(settings.ResolutionY / perpWallDist);

	int texIdx = int(GetWorldValue(mapX, mapY) - 1);

	double wallX;

	if (side == 0) wallX = player.position.y + perpWallDist * rayDirY;
	else wallX = player.position.x + perpWallDist * rayDirX;
	wallX -= floor(wallX);

	int texX = int(wallX * double(world.geometryTileWidth));

	if (side == 0 && rayDirX > 0) texX = int(world.geometryTileWidth) - texX - 1;
	if (side == 1 && rayDirY < 0) texX = int(world.geometryTileWidth) - texX - 1;

	//Store calculations
	sliceData.data[x * 3 + 0] = lineHeight;
	sliceData.data[x * 3 + 1] = texIdx;
	sliceData.data[x * 3 + 2] = texX;
}

void main()
{
	ivec2 iCoords = ivec2(gl_GlobalInvocationID.xy);

	PrecomputeSlice(iCoords.x);
}
