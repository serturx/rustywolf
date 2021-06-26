//Author: Jerrit Gläsker

#version 430
layout(local_size_x = 1, local_size_y = 1) in;
layout(rgba32f, binding = 0) uniform image2D img;

layout(std430, binding = 1) buffer Settings {
	uint ResolutionX;
	uint ResolutionY;
} settings;

layout(std430, binding = 3) buffer World {
	uint ceilingTexIdx;
	uint floorTexIdx;
	uint geometryTileWidth;
	uint stride;
	uint[] geometry;
} world;

layout(binding = 4) uniform sampler2DArray texAtlas;

layout(std430, binding = 6) buffer CaFData {
	float data[];
} cafData;

float LineToLod(int y)
{
	int p = y - int(settings.ResolutionY) / 2;
	float posZ = 0.5 * float(settings.ResolutionY);
	float rowDistance = posZ / p;

	float lod = 0.0;

	for (float dist = rowDistance; dist >= 0 && lod < 5.0; dist -= 1)
		lod += 0.1;
	return lod;
}

vec4 GetAtlasColor(int textureIdx, int x, int y, float lod)
{
	float x_n = float(x) / float(world.geometryTileWidth);
	float y_n = float(y) / float(world.geometryTileWidth);

	float x_centerOffset = 1.0 / (float(world.geometryTileWidth) * 2.0);
	float y_centerOffset = 1.0 / (float(world.geometryTileWidth) * 2.0);

	return textureLod(texAtlas, vec3(x_n + x_centerOffset, y_n + y_centerOffset, textureIdx), lod);
}

void Compute(ivec2 iCoords)
{
	float floorX = cafData.data[iCoords.y * 4 + 0];
	float floorY = cafData.data[iCoords.y * 4 + 1];
	float floorStepX = cafData.data[iCoords.y * 4 + 2];
	float floorStepY = cafData.data[iCoords.y * 4 + 3];

	floorX += iCoords.x * floorStepX;
	floorY += iCoords.x * floorStepY;

	int cellX = int(floorX);
	int cellY = int(floorY);

	int tx = int(world.geometryTileWidth * (floorX - cellX)) & int(world.geometryTileWidth - 1);
	int ty = int(world.geometryTileWidth * (floorY - cellY)) & int(world.geometryTileWidth - 1);

	int floorTextureIdx = int(world.floorTexIdx);
	int ceilingTextureIdx = int(world.ceilingTexIdx);

	//floor
	vec4 color = GetAtlasColor(floorTextureIdx - 1, tx, ty, LineToLod(iCoords.y));

	imageStore(img, iCoords, color);
	//ceiling
	color = GetAtlasColor(ceilingTextureIdx - 1, tx, ty, LineToLod(iCoords.y));
	ivec2 pos = ivec2(iCoords.x, settings.ResolutionY - iCoords.y - 1);

	imageStore(img, pos, color);
}

void main()
{
	ivec2 iCoords = ivec2(gl_GlobalInvocationID.xy);

	Compute(iCoords);
}
