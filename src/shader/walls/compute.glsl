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

layout(std430, binding = 5) buffer SliceData {
	int data[];
} sliceData;

float LineHeightToLod(int lineHeight)
{
	double perpWallDist = double(settings.ResolutionX) / double(lineHeight);

	float lod = 0.0;

	for (double dist = perpWallDist; dist >= 0 && lod < 10.0; dist -= 1)
		lod += 0.025;
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

vec4 ProcessSlice(ivec2 iCoords)
{
	int x = iCoords.x;

	int lineHeight = sliceData.data[x * 3 + 0];
	int texIdx = sliceData.data[x * 3 + 1];
	int texX = sliceData.data[x * 3 + 2];

	int drawStart = int(-lineHeight / 2.0 + int(settings.ResolutionY) / 2.0);

	if (drawStart < 0) drawStart = 0;
	int drawEnd = int(lineHeight / 2.0 + int(settings.ResolutionY) / 2.0);

	if (drawEnd >= settings.ResolutionY) drawEnd = int(settings.ResolutionY - 1);

	int y = iCoords.y;
	double texStep = 1.0 * int(world.geometryTileWidth) / lineHeight;
	double texPos = (y - settings.ResolutionY / 2 + lineHeight / 2) * texStep;

	if (y < drawStart || y >= drawEnd) {
		return imageLoad(img, iCoords);
	} else {
		int texY = int(texPos) & (int(world.geometryTileWidth) - 1);
		return GetAtlasColor(int(texIdx), texX, texY, LineHeightToLod(lineHeight));
	}
}

void main()
{
	ivec2 iCoords = ivec2(gl_GlobalInvocationID.xy);
	vec4 color = ProcessSlice(iCoords);

	imageStore(img, iCoords, color);
}
