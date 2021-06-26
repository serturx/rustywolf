//Author: Jerrit Gläsker

#version 430
layout(local_size_x = 1, local_size_y = 1) in;

layout(std430, binding = 1) buffer Settings {
	uint ResolutionX;
	uint ResolutionY;
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

layout(std430, binding = 6) buffer CaFData {
	float data[];
} cafData;

void CalculateCaF(int y)
{
	float rayDirX0 = player.direction.x - player.camPlane.x;
	float rayDirY0 = player.direction.y - player.camPlane.y;
	float rayDirX1 = player.direction.x + player.camPlane.x;
	float rayDirY1 = player.direction.y + player.camPlane.y;

	int p = y - int(settings.ResolutionY) / 2;

	float posZ = 0.5 * float(settings.ResolutionY);

	float rowDistance = posZ / p;

	float floorStepX = rowDistance * (rayDirX1 - rayDirX0) / settings.ResolutionX;
	float floorStepY = rowDistance * (rayDirY1 - rayDirY0) / settings.ResolutionX;

	float floorX = player.position.x + rowDistance * rayDirX0;
	float floorY = player.position.y + rowDistance * rayDirY0;

	cafData.data[y * 4 + 0] = floorX;
	cafData.data[y * 4 + 1] = floorY;
	cafData.data[y * 4 + 2] = floorStepX;
	cafData.data[y * 4 + 3] = floorStepY;
}

void main()
{
	ivec2 iCoords = ivec2(gl_GlobalInvocationID.xy);

	CalculateCaF(iCoords.y);
}
