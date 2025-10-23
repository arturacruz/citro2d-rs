#include "/opt/devkitpro/libctru/include/3ds.h"
#include "/opt/devkitpro/devkitARM/arm-none-eabi/include/errno.h"

// Static wrappers

u32 IPC_MakeHeader__extern(u16 command_id, unsigned int normal_params, unsigned int translate_params) { return IPC_MakeHeader(command_id, normal_params, translate_params); }
u32 IPC_Desc_SharedHandles__extern(unsigned int number) { return IPC_Desc_SharedHandles(number); }
u32 IPC_Desc_MoveHandles__extern(unsigned int number) { return IPC_Desc_MoveHandles(number); }
u32 IPC_Desc_CurProcessId__extern(void) { return IPC_Desc_CurProcessId(); }
u32 IPC_Desc_CurProcessHandle__extern(void) { return IPC_Desc_CurProcessHandle(); }
u32 IPC_Desc_StaticBuffer__extern(size_t size, unsigned int buffer_id) { return IPC_Desc_StaticBuffer(size, buffer_id); }
u32 IPC_Desc_PXIBuffer__extern(size_t size, unsigned int buffer_id, bool is_read_only) { return IPC_Desc_PXIBuffer(size, buffer_id, is_read_only); }
u32 IPC_Desc_Buffer__extern(size_t size, IPC_BufferRights rights) { return IPC_Desc_Buffer(size, rights); }
void * getThreadLocalStorage__extern(void) { return getThreadLocalStorage(); }
u32 * getThreadCommandBuffer__extern(void) { return getThreadCommandBuffer(); }
u32 * getThreadStaticBuffers__extern(void) { return getThreadStaticBuffers(); }
void dmaDeviceConfigInitDefault__extern(DmaDeviceConfig *cfg) { dmaDeviceConfigInitDefault(cfg); }
void dmaConfigInitDefault__extern(DmaConfig *cfg) { dmaConfigInitDefault(cfg); }
u32 osGetFirmVersion__extern(void) { return osGetFirmVersion(); }
u32 osGetKernelVersion__extern(void) { return osGetKernelVersion(); }
u32 osGetSystemCoreVersion__extern(void) { return osGetSystemCoreVersion(); }
u32 osGetApplicationMemType__extern(void) { return osGetApplicationMemType(); }
u32 osGetMemRegionSize__extern(MemRegion region) { return osGetMemRegionSize(region); }
u32 osGetMemRegionUsed__extern(MemRegion region) { return osGetMemRegionUsed(region); }
u32 osGetMemRegionFree__extern(MemRegion region) { return osGetMemRegionFree(region); }
void osTickCounterStart__extern(TickCounter *cnt) { osTickCounterStart(cnt); }
void osTickCounterUpdate__extern(TickCounter *cnt) { osTickCounterUpdate(cnt); }
u8 osGetWifiStrength__extern(void) { return osGetWifiStrength(); }
float osGet3DSliderState__extern(void) { return osGet3DSliderState(); }
bool osIsHeadsetConnected__extern(void) { return osIsHeadsetConnected(); }
void __dsb__extern(void) { __dsb(); }
void __dmb__extern(void) { __dmb(); }
void __isb__extern(void) { __isb(); }
void __clrex__extern(void) { __clrex(); }
s32 __ldrex__extern(s32 *addr) { return __ldrex(addr); }
bool __strex__extern(s32 *addr, s32 val) { return __strex(addr, val); }
u16 __ldrexh__extern(u16 *addr) { return __ldrexh(addr); }
bool __strexh__extern(u16 *addr, u16 val) { return __strexh(addr, val); }
u8 __ldrexb__extern(u8 *addr) { return __ldrexb(addr); }
bool __strexb__extern(u8 *addr, u8 val) { return __strexb(addr, val); }
void CondVar_Signal__extern(CondVar *cv) { CondVar_Signal(cv); }
void CondVar_Broadcast__extern(CondVar *cv) { CondVar_Broadcast(cv); }
void threadOnException__extern(ExceptionHandler handler, void *stack_top, ERRF_ExceptionData *exception_data) { threadOnException(handler, stack_top, exception_data); }
unsigned int gspGetBytesPerPixel__extern(GSPGPU_FramebufferFormat format) { return gspGetBytesPerPixel(format); }
bool envIsHomebrew__extern(void) { return envIsHomebrew(); }
u32 envGetAptAppId__extern(void) { return envGetAptAppId(); }
u32 envGetHeapSize__extern(void) { return envGetHeapSize(); }
u32 envGetLinearHeapSize__extern(void) { return envGetLinearHeapSize(); }
const char * envGetSystemArgList__extern(void) { return envGetSystemArgList(); }
u32 envGetSystemRunFlags__extern(void) { return envGetSystemRunFlags(); }
bool decompress__extern(void *output, size_t size, decompressCallback callback, void *userdata, size_t insize) { return decompress(output, size, callback, userdata, insize); }
bool decompress_LZSS__extern(void *output, size_t size, decompressCallback callback, void *userdata, size_t insize) { return decompress_LZSS(output, size, callback, userdata, insize); }
bool decompress_LZ11__extern(void *output, size_t size, decompressCallback callback, void *userdata, size_t insize) { return decompress_LZ11(output, size, callback, userdata, insize); }
bool decompress_Huff__extern(size_t bits, void *output, size_t size, decompressCallback callback, void *userdata, size_t insize) { return decompress_Huff(bits, output, size, callback, userdata, insize); }
bool decompress_RLE__extern(void *output, size_t size, decompressCallback callback, void *userdata, size_t insize) { return decompress_RLE(output, size, callback, userdata, insize); }
APT_AppletAttr aptMakeAppletAttr__extern(APT_AppletPos pos, bool manualGpuRights, bool manualDspRights) { return aptMakeAppletAttr(pos, manualGpuRights, manualDspRights); }
bool aptIsHomePressed__extern(void) { return aptIsHomePressed(); }
void aptHandleJumpToHome__extern(void) { aptHandleJumpToHome(); }
NS_APPID aptGetMenuAppID__extern(void) { return aptGetMenuAppID(); }
u32 CSND_VOL__extern(float vol, float pan) { return CSND_VOL(vol, pan); }
s32 ptmSysmGetNotificationAckValue__extern(u32 id) { return ptmSysmGetNotificationAckValue(id); }
u8 pxiDevMakeTransferOption__extern(FS_CardSpiBaudRate baudRate, FS_CardSpiBusMode busMode) { return pxiDevMakeTransferOption(baudRate, busMode); }
u64 pxiDevMakeWaitOperation__extern(PXIDEV_WaitType waitType, PXIDEV_DeassertType deassertType, u64 timeout) { return pxiDevMakeWaitOperation(waitType, deassertType, timeout); }
Result QTMU_GetRawTrackingData__extern(QtmRawTrackingData *outData) { return QTMU_GetRawTrackingData(outData); }
Result QTMU_GetTrackingData__extern(QtmTrackingData *outData) { return QTMU_GetTrackingData(outData); }
void gxCmdQueueSetCallback__extern(gxCmdQueue_s *queue, void (*callback) (gxCmdQueue_s *), void *user) { gxCmdQueueSetCallback(queue, callback, user); }
void GPUCMD_SetBuffer__extern(u32 *adr, u32 size, u32 offset) { GPUCMD_SetBuffer(adr, size, offset); }
void GPUCMD_SetBufferOffset__extern(u32 offset) { GPUCMD_SetBufferOffset(offset); }
void GPUCMD_GetBuffer__extern(u32 **addr, u32 *size, u32 *offset) { GPUCMD_GetBuffer(addr, size, offset); }
void GPUCMD_AddSingleParam__extern(u32 header, u32 param) { GPUCMD_AddSingleParam(header, param); }
void swkbdSetPasswordMode__extern(SwkbdState *swkbd, SwkbdPasswordMode mode) { swkbdSetPasswordMode(swkbd, mode); }
void swkbdSetValidation__extern(SwkbdState *swkbd, SwkbdValidInput validInput, u32 filterFlags, int maxDigits) { swkbdSetValidation(swkbd, validInput, filterFlags, maxDigits); }
void swkbdSetNumpadKeys__extern(SwkbdState *swkbd, int left, int right) { swkbdSetNumpadKeys(swkbd, left, right); }
SwkbdResult swkbdGetResult__extern(SwkbdState *swkbd) { return swkbdGetResult(swkbd); }
void miiSelectorSetInitialIndex__extern(MiiSelectorConf *conf, u32 index) { miiSelectorSetInitialIndex(conf, index); }
Result romfsInit__extern(void) { return romfsInit(); }
Result romfsExit__extern(void) { return romfsExit(); }
CFNT_s * fontGetSystemFont__extern(void) { return fontGetSystemFont(); }
FINF_s * fontGetInfo__extern(CFNT_s *font) { return fontGetInfo(font); }
TGLP_s * fontGetGlyphInfo__extern(CFNT_s *font) { return fontGetGlyphInfo(font); }
void * fontGetGlyphSheetTex__extern(CFNT_s *font, int sheetIndex) { return fontGetGlyphSheetTex(font, sheetIndex); }
int link3dsStdio__extern(void) { return link3dsStdio(); }
int link3dsStdioForDebug__extern(void) { return link3dsStdioForDebug(); }
