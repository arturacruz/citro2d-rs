#include "/opt/devkitpro/libctru/include/3ds.h"
#include "/opt/devkitpro/libctru/include/citro2d.h"

// Static wrappers

C3D_IVec IVec_Pack__extern(u8 x, u8 y, u8 z, u8 w) { return IVec_Pack(x, y, z, w); }
C3D_FVec FVec4_New__extern(float x, float y, float z, float w) { return FVec4_New(x, y, z, w); }
C3D_FVec FVec4_Add__extern(C3D_FVec lhs, C3D_FVec rhs) { return FVec4_Add(lhs, rhs); }
C3D_FVec FVec4_Subtract__extern(C3D_FVec lhs, C3D_FVec rhs) { return FVec4_Subtract(lhs, rhs); }
C3D_FVec FVec4_Negate__extern(C3D_FVec v) { return FVec4_Negate(v); }
C3D_FVec FVec4_Scale__extern(C3D_FVec v, float s) { return FVec4_Scale(v, s); }
C3D_FVec FVec4_PerspDivide__extern(C3D_FVec v) { return FVec4_PerspDivide(v); }
float FVec4_Dot__extern(C3D_FVec lhs, C3D_FVec rhs) { return FVec4_Dot(lhs, rhs); }
float FVec4_Magnitude__extern(C3D_FVec v) { return FVec4_Magnitude(v); }
C3D_FVec FVec4_Normalize__extern(C3D_FVec v) { return FVec4_Normalize(v); }
C3D_FVec FVec3_New__extern(float x, float y, float z) { return FVec3_New(x, y, z); }
float FVec3_Dot__extern(C3D_FVec lhs, C3D_FVec rhs) { return FVec3_Dot(lhs, rhs); }
float FVec3_Magnitude__extern(C3D_FVec v) { return FVec3_Magnitude(v); }
C3D_FVec FVec3_Normalize__extern(C3D_FVec v) { return FVec3_Normalize(v); }
C3D_FVec FVec3_Add__extern(C3D_FVec lhs, C3D_FVec rhs) { return FVec3_Add(lhs, rhs); }
C3D_FVec FVec3_Subtract__extern(C3D_FVec lhs, C3D_FVec rhs) { return FVec3_Subtract(lhs, rhs); }
float FVec3_Distance__extern(C3D_FVec lhs, C3D_FVec rhs) { return FVec3_Distance(lhs, rhs); }
C3D_FVec FVec3_Scale__extern(C3D_FVec v, float s) { return FVec3_Scale(v, s); }
C3D_FVec FVec3_Negate__extern(C3D_FVec v) { return FVec3_Negate(v); }
C3D_FVec FVec3_Cross__extern(C3D_FVec lhs, C3D_FVec rhs) { return FVec3_Cross(lhs, rhs); }
void Mtx_Zeros__extern(C3D_Mtx *out) { Mtx_Zeros(out); }
void Mtx_Copy__extern(C3D_Mtx *out, const C3D_Mtx *in) { Mtx_Copy(out, in); }
void Mtx_Diagonal__extern(C3D_Mtx *out, float x, float y, float z, float w) { Mtx_Diagonal(out, x, y, z, w); }
void Mtx_Identity__extern(C3D_Mtx *out) { Mtx_Identity(out); }
void Mtx_Add__extern(C3D_Mtx *out, const C3D_Mtx *lhs, const C3D_Mtx *rhs) { Mtx_Add(out, lhs, rhs); }
void Mtx_Subtract__extern(C3D_Mtx *out, const C3D_Mtx *lhs, const C3D_Mtx *rhs) { Mtx_Subtract(out, lhs, rhs); }
C3D_FVec Mtx_MultiplyFVecH__extern(const C3D_Mtx *mtx, C3D_FVec v) { return Mtx_MultiplyFVecH(mtx, v); }
C3D_FQuat Quat_Identity__extern(void) { return Quat_Identity(); }
C3D_FQuat Quat_Conjugate__extern(C3D_FQuat q) { return Quat_Conjugate(q); }
C3D_FQuat Quat_Inverse__extern(C3D_FQuat q) { return Quat_Inverse(q); }
C3D_FVec FVec3_CrossQuat__extern(C3D_FVec v, C3D_FQuat q) { return FVec3_CrossQuat(v, q); }
C3D_Mtx * MtxStack_Cur__extern(C3D_MtxStack *stk) { return MtxStack_Cur(stk); }
C3D_FVec * C3D_FVUnifWritePtr__extern(GPU_SHADER_TYPE type, int id, int size) { return C3D_FVUnifWritePtr(type, id, size); }
C3D_IVec * C3D_IVUnifWritePtr__extern(GPU_SHADER_TYPE type, int id) { return C3D_IVUnifWritePtr(type, id); }
void C3D_FVUnifMtxNx4__extern(GPU_SHADER_TYPE type, int id, const C3D_Mtx *mtx, int num) { C3D_FVUnifMtxNx4(type, id, mtx, num); }
void C3D_FVUnifMtx4x4__extern(GPU_SHADER_TYPE type, int id, const C3D_Mtx *mtx) { C3D_FVUnifMtx4x4(type, id, mtx); }
void C3D_FVUnifMtx3x4__extern(GPU_SHADER_TYPE type, int id, const C3D_Mtx *mtx) { C3D_FVUnifMtx3x4(type, id, mtx); }
void C3D_FVUnifMtx2x4__extern(GPU_SHADER_TYPE type, int id, const C3D_Mtx *mtx) { C3D_FVUnifMtx2x4(type, id, mtx); }
void C3D_FVUnifSet__extern(GPU_SHADER_TYPE type, int id, float x, float y, float z, float w) { C3D_FVUnifSet(type, id, x, y, z, w); }
void C3D_IVUnifSet__extern(GPU_SHADER_TYPE type, int id, int x, int y, int z, int w) { C3D_IVUnifSet(type, id, x, y, z, w); }
void C3D_BoolUnifSet__extern(GPU_SHADER_TYPE type, int id, bool value) { C3D_BoolUnifSet(type, id, value); }
void C3D_ImmDrawRestartPrim__extern(void) { C3D_ImmDrawRestartPrim(); }
void C3D_FixedAttribSet__extern(int id, float x, float y, float z, float w) { C3D_FixedAttribSet(id, x, y, z, w); }
void C3D_TexEnvInit__extern(C3D_TexEnv *env) { C3D_TexEnvInit(env); }
void C3D_TexEnvSrc__extern(C3D_TexEnv *env, C3D_TexEnvMode mode, GPU_TEVSRC s1, GPU_TEVSRC s2, GPU_TEVSRC s3) { C3D_TexEnvSrc(env, mode, s1, s2, s3); }
void C3D_TexEnvOpRgb__extern(C3D_TexEnv *env, GPU_TEVOP_RGB o1, GPU_TEVOP_RGB o2, GPU_TEVOP_RGB o3) { C3D_TexEnvOpRgb(env, o1, o2, o3); }
void C3D_TexEnvOpAlpha__extern(C3D_TexEnv *env, GPU_TEVOP_A o1, GPU_TEVOP_A o2, GPU_TEVOP_A o3) { C3D_TexEnvOpAlpha(env, o1, o2, o3); }
void C3D_TexEnvFunc__extern(C3D_TexEnv *env, C3D_TexEnvMode mode, GPU_COMBINEFUNC param) { C3D_TexEnvFunc(env, mode, param); }
void C3D_TexEnvColor__extern(C3D_TexEnv *env, u32 color) { C3D_TexEnvColor(env, color); }
void C3D_TexEnvScale__extern(C3D_TexEnv *env, int mode, GPU_TEVSCALE param) { C3D_TexEnvScale(env, mode, param); }
int C3D_TexCalcMaxLevel__extern(u32 width, u32 height) { return C3D_TexCalcMaxLevel(width, height); }
u32 C3D_TexCalcLevelSize__extern(u32 size, int level) { return C3D_TexCalcLevelSize(size, level); }
u32 C3D_TexCalcTotalSize__extern(u32 size, int maxLevel) { return C3D_TexCalcTotalSize(size, maxLevel); }
bool C3D_TexInit__extern(C3D_Tex *tex, u16 width, u16 height, GPU_TEXCOLOR format) { return C3D_TexInit(tex, width, height, format); }
bool C3D_TexInitMipmap__extern(C3D_Tex *tex, u16 width, u16 height, GPU_TEXCOLOR format) { return C3D_TexInitMipmap(tex, width, height, format); }
bool C3D_TexInitCube__extern(C3D_Tex *tex, C3D_TexCube *cube, u16 width, u16 height, GPU_TEXCOLOR format) { return C3D_TexInitCube(tex, cube, width, height, format); }
bool C3D_TexInitVRAM__extern(C3D_Tex *tex, u16 width, u16 height, GPU_TEXCOLOR format) { return C3D_TexInitVRAM(tex, width, height, format); }
bool C3D_TexInitShadow__extern(C3D_Tex *tex, u16 width, u16 height) { return C3D_TexInitShadow(tex, width, height); }
bool C3D_TexInitShadowCube__extern(C3D_Tex *tex, C3D_TexCube *cube, u16 width, u16 height) { return C3D_TexInitShadowCube(tex, cube, width, height); }
GPU_TEXTURE_MODE_PARAM C3D_TexGetType__extern(C3D_Tex *tex) { return C3D_TexGetType(tex); }
void * C3D_TexGetImagePtr__extern(C3D_Tex *tex, void *data, int level, u32 *size) { return C3D_TexGetImagePtr(tex, data, level, size); }
void * C3D_Tex2DGetImagePtr__extern(C3D_Tex *tex, int level, u32 *size) { return C3D_Tex2DGetImagePtr(tex, level, size); }
void * C3D_TexCubeGetImagePtr__extern(C3D_Tex *tex, GPU_TEXFACE face, int level, u32 *size) { return C3D_TexCubeGetImagePtr(tex, face, level, size); }
void C3D_TexUpload__extern(C3D_Tex *tex, const void *data) { C3D_TexUpload(tex, data); }
void C3D_TexSetFilter__extern(C3D_Tex *tex, GPU_TEXTURE_FILTER_PARAM magFilter, GPU_TEXTURE_FILTER_PARAM minFilter) { C3D_TexSetFilter(tex, magFilter, minFilter); }
void C3D_TexSetFilterMipmap__extern(C3D_Tex *tex, GPU_TEXTURE_FILTER_PARAM filter) { C3D_TexSetFilterMipmap(tex, filter); }
void C3D_TexSetWrap__extern(C3D_Tex *tex, GPU_TEXTURE_WRAP_PARAM wrapS, GPU_TEXTURE_WRAP_PARAM wrapT) { C3D_TexSetWrap(tex, wrapS, wrapT); }
void C3D_TexSetLodBias__extern(C3D_Tex *tex, float lodBias) { C3D_TexSetLodBias(tex, lodBias); }
void C3D_ProcTexClamp__extern(C3D_ProcTex *pt, GPU_PROCTEX_CLAMP u, GPU_PROCTEX_CLAMP v) { C3D_ProcTexClamp(pt, u, v); }
void C3D_ProcTexCombiner__extern(C3D_ProcTex *pt, bool separate, GPU_PROCTEX_MAPFUNC rgb, GPU_PROCTEX_MAPFUNC alpha) { C3D_ProcTexCombiner(pt, separate, rgb, alpha); }
void C3D_ProcTexNoiseEnable__extern(C3D_ProcTex *pt, bool enable) { C3D_ProcTexNoiseEnable(pt, enable); }
void C3D_ProcTexShift__extern(C3D_ProcTex *pt, GPU_PROCTEX_SHIFT u, GPU_PROCTEX_SHIFT v) { C3D_ProcTexShift(pt, u, v); }
void C3D_ProcTexFilter__extern(C3D_ProcTex *pt, GPU_PROCTEX_FILTER min) { C3D_ProcTexFilter(pt, min); }
float quadratic_dist_attn__extern(float dist, float linear, float quad_t) { return quadratic_dist_attn(dist, linear, quad_t); }
float spot_step__extern(float angle, float cutoff) { return spot_step(angle, cutoff); }
void C3D_LightColor__extern(C3D_Light *light, float r, float g, float b) { C3D_LightColor(light, r, g, b); }
float FogLut_CalcZ__extern(float depth, float near, float far) { return FogLut_CalcZ(depth, near, far); }
void C3D_FrameBufAttrib__extern(C3D_FrameBuf *fb, u16 width, u16 height, bool block32) { C3D_FrameBufAttrib(fb, width, height, block32); }
void C3D_FrameBufColor__extern(C3D_FrameBuf *fb, void *buf, GPU_COLORBUF fmt) { C3D_FrameBufColor(fb, buf, fmt); }
void C3D_FrameBufDepth__extern(C3D_FrameBuf *fb, void *buf, GPU_DEPTHBUF fmt) { C3D_FrameBufDepth(fb, buf, fmt); }
void C3D_RenderTargetDetachOutput__extern(C3D_RenderTarget *target) { C3D_RenderTargetDetachOutput(target); }
void C3D_RenderTargetClear__extern(C3D_RenderTarget *target, C3D_ClearBits clearBits, u32 clearColor, u32 clearDepth) { C3D_RenderTargetClear(target, clearBits, clearColor, clearDepth); }
bool Tex3DS_SubTextureRotated__extern(const Tex3DS_SubTexture *subtex) { return Tex3DS_SubTextureRotated(subtex); }
void Tex3DS_SubTextureBottomLeft__extern(const Tex3DS_SubTexture *subtex, float *u, float *v) { Tex3DS_SubTextureBottomLeft(subtex, u, v); }
void Tex3DS_SubTextureBottomRight__extern(const Tex3DS_SubTexture *subtex, float *u, float *v) { Tex3DS_SubTextureBottomRight(subtex, u, v); }
void Tex3DS_SubTextureTopLeft__extern(const Tex3DS_SubTexture *subtex, float *u, float *v) { Tex3DS_SubTextureTopLeft(subtex, u, v); }
void Tex3DS_SubTextureTopRight__extern(const Tex3DS_SubTexture *subtex, float *u, float *v) { Tex3DS_SubTextureTopRight(subtex, u, v); }
float C2D_Clamp__extern(float x, float min, float max) { return C2D_Clamp(x, min, max); }
u8 C2D_FloatToU8__extern(float x) { return C2D_FloatToU8(x); }
u32 C2D_Color32__extern(u8 r, u8 g, u8 b, u8 a) { return C2D_Color32(r, g, b, a); }
u32 C2D_Color32f__extern(float r, float g, float b, float a) { return C2D_Color32f(r, g, b, a); }
void C2D_SetImageTint__extern(C2D_ImageTint *tint, C2D_Corner corner, u32 color, float blend) { C2D_SetImageTint(tint, corner, color, blend); }
void C2D_PlainImageTint__extern(C2D_ImageTint *tint, u32 color, float blend) { C2D_PlainImageTint(tint, color, blend); }
void C2D_AlphaImageTint__extern(C2D_ImageTint *tint, float alpha) { C2D_AlphaImageTint(tint, alpha); }
void C2D_TopImageTint__extern(C2D_ImageTint *tint, u32 color, float blend) { C2D_TopImageTint(tint, color, blend); }
void C2D_BottomImageTint__extern(C2D_ImageTint *tint, u32 color, float blend) { C2D_BottomImageTint(tint, color, blend); }
void C2D_LeftImageTint__extern(C2D_ImageTint *tint, u32 color, float blend) { C2D_LeftImageTint(tint, color, blend); }
void C2D_RightImageTint__extern(C2D_ImageTint *tint, u32 color, float blend) { C2D_RightImageTint(tint, color, blend); }
void C2D_SceneTarget__extern(C3D_RenderTarget *target) { C2D_SceneTarget(target); }
void C2D_ViewRotateDegrees__extern(float rotation) { C2D_ViewRotateDegrees(rotation); }
void C2D_SceneBegin__extern(C3D_RenderTarget *target) { C2D_SceneBegin(target); }
bool C2D_DrawImageAt__extern(C2D_Image img, float x, float y, float depth, const C2D_ImageTint *tint, float scaleX, float scaleY) { return C2D_DrawImageAt(img, x, y, depth, tint, scaleX, scaleY); }
bool C2D_DrawImageAtRotated__extern(C2D_Image img, float x, float y, float depth, float angle, const C2D_ImageTint *tint, float scaleX, float scaleY) { return C2D_DrawImageAtRotated(img, x, y, depth, angle, tint, scaleX, scaleY); }
bool C2D_DrawRectSolid__extern(float x, float y, float z, float w, float h, u32 clr) { return C2D_DrawRectSolid(x, y, z, w, h, clr); }
bool C2D_DrawEllipseSolid__extern(float x, float y, float z, float w, float h, u32 clr) { return C2D_DrawEllipseSolid(x, y, z, w, h, clr); }
bool C2D_DrawCircle__extern(float x, float y, float z, float radius, u32 clr0, u32 clr1, u32 clr2, u32 clr3) { return C2D_DrawCircle(x, y, z, radius, clr0, clr1, clr2, clr3); }
bool C2D_DrawCircleSolid__extern(float x, float y, float z, float radius, u32 clr) { return C2D_DrawCircleSolid(x, y, z, radius, clr); }
void C2D_SpriteFromImage__extern(C2D_Sprite *sprite, C2D_Image image) { C2D_SpriteFromImage(sprite, image); }
void C2D_SpriteFromSheet__extern(C2D_Sprite *sprite, C2D_SpriteSheet sheet, size_t index) { C2D_SpriteFromSheet(sprite, sheet, index); }
void C2D_SpriteScale__extern(C2D_Sprite *sprite, float x, float y) { C2D_SpriteScale(sprite, x, y); }
void C2D_SpriteRotate__extern(C2D_Sprite *sprite, float radians) { C2D_SpriteRotate(sprite, radians); }
void C2D_SpriteRotateDegrees__extern(C2D_Sprite *sprite, float degrees) { C2D_SpriteRotateDegrees(sprite, degrees); }
void C2D_SpriteMove__extern(C2D_Sprite *sprite, float x, float y) { C2D_SpriteMove(sprite, x, y); }
void C2D_SpriteSetScale__extern(C2D_Sprite *sprite, float x, float y) { C2D_SpriteSetScale(sprite, x, y); }
void C2D_SpriteSetRotation__extern(C2D_Sprite *sprite, float radians) { C2D_SpriteSetRotation(sprite, radians); }
void C2D_SpriteSetRotationDegrees__extern(C2D_Sprite *sprite, float degrees) { C2D_SpriteSetRotationDegrees(sprite, degrees); }
void C2D_SpriteSetCenter__extern(C2D_Sprite *sprite, float x, float y) { C2D_SpriteSetCenter(sprite, x, y); }
void C2D_SpriteSetCenterRaw__extern(C2D_Sprite *sprite, float x, float y) { C2D_SpriteSetCenterRaw(sprite, x, y); }
void C2D_SpriteSetPos__extern(C2D_Sprite *sprite, float x, float y) { C2D_SpriteSetPos(sprite, x, y); }
void C2D_SpriteSetDepth__extern(C2D_Sprite *sprite, float depth) { C2D_SpriteSetDepth(sprite, depth); }
bool C2D_DrawSprite__extern(const C2D_Sprite *sprite) { return C2D_DrawSprite(sprite); }
bool C2D_DrawSpriteTinted__extern(const C2D_Sprite *sprite, const C2D_ImageTint *tint) { return C2D_DrawSpriteTinted(sprite, tint); }
