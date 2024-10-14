# aslr-cop

Quick check for ASLR and DEP violations!

## Usage

```bash
$ cargo run -qr -- -d /mnt/win11/Windows/System32/
"winload.efi"
    ASLR disabled DEP disabled (0x0)

"winload.exe"
    ASLR disabled DEP disabled (0x0)

"winresume.efi"
    ASLR disabled DEP disabled (0x0)

"winresume.exe"
    ASLR disabled DEP disabled (0x0)

"d3dx9_30.dll"
    ASLR disabled DEP disabled (0x0)

"D3DCompiler_33.dll"
    ASLR enabled DEP disabled (0x40)

"D3DCompiler_34.dll"
    ASLR enabled DEP disabled (0x40)

"D3DCompiler_35.dll"
    ASLR enabled DEP disabled (0x40)

"D3DCompiler_36.dll"
    ASLR enabled DEP disabled (0x40)

"D3DCompiler_37.dll"
    ASLR enabled DEP disabled (0x40)

"D3DCompiler_38.dll"
    ASLR enabled DEP disabled (0x40)

"D3DCompiler_39.dll"
    ASLR enabled DEP disabled (0x40)

"D3DCompiler_40.dll"
    ASLR enabled DEP disabled (0x40)

"D3DCompiler_41.dll"
    ASLR enabled DEP disabled (0x40)

"D3DCompiler_42.dll"
    ASLR enabled DEP disabled (0x40)

"d3dcsx_42.dll"
    ASLR enabled DEP disabled (0x40)

"hvloader.dll"
    ASLR disabled DEP disabled (0x0)

"RTCOM64.dll"
    ASLR disabled DEP disabled (0x0)

"PresentationCFFRasterizerNative_v0300.dll"
    ASLR enabled DEP disabled (0x40)

"PresentationNative_v0300.dll"
    ASLR enabled DEP disabled (0x40)

"RCoRes64.dat"
    ASLR disabled DEP disabled (0x0)

"RTSnMg64.cpl"
    ASLR disabled DEP disabled (0x0)

"SecConfig.efi"
    ASLR disabled DEP disabled (0x0)

"SRSTSX64.dll"
    ASLR disabled DEP disabled (0x0)

"SRSWOW64.dll"
    ASLR disabled DEP disabled (0x0)

"stdole32.tlb"
    ASLR disabled DEP disabled (0x0)

"SynTPEnh.exe"
    ASLR disabled DEP enabled (0x8120)

"TsWpfWrp.exe"
    ASLR enabled DEP disabled (0x440)

"WdfCoInstaller01009.dll"
    ASLR enabled DEP disabled (0x40)

"xinput1_1.dll"
    ASLR disabled DEP disabled (0x0)

"xinput1_2.dll"
    ASLR disabled DEP disabled (0x0)

"xinput1_3.dll"
    ASLR disabled DEP disabled (0x0)

"xactengine2_2.dll"
    ASLR disabled DEP disabled (0x0)

"GfxValDisplayLog.bin"
Malformed(
    "Object is too small.",
)

"mscories.dll"
    ASLR enabled DEP disabled (0x40)

"tcblaunch.exe"
    ASLR disabled DEP disabled (0x0)

"tcbloader.dll"
    ASLR disabled DEP disabled (0x0)

"nvdebugdump.exe"
    ASLR disabled DEP disabled (0x8000)

"RtlCPAPI64.dll"
    ASLR disabled DEP disabled (0x0)

"libusb0.dll"
    ASLR enabled DEP disabled (0x40)

"libusbK.dll"
    ASLR enabled DEP disabled (0x40)

"x3daudio1_0.dll"
    ASLR disabled DEP disabled (0x0)

"x3daudio1_1.dll"
    ASLR disabled DEP disabled (0x0)

"X3DAudio1_2.dll"
    ASLR disabled DEP disabled (0x0)

"X3DAudio1_3.dll"
    ASLR enabled DEP disabled (0x40)

"X3DAudio1_4.dll"
    ASLR enabled DEP disabled (0x40)

"X3DAudio1_5.dll"
    ASLR enabled DEP disabled (0x40)

"X3DAudio1_6.dll"
    ASLR enabled DEP disabled (0x40)

"X3DAudio1_7.dll"
    ASLR enabled DEP disabled (0x40)

"X3DAudioD1_7.dll"
    ASLR enabled DEP disabled (0x40)

"d3dx9_31.dll"
    ASLR disabled DEP disabled (0x0)

"d3dx9_32.dll"
    ASLR enabled DEP disabled (0x40)

"d3dx9_33.dll"
    ASLR enabled DEP disabled (0x40)

"d3dx9_34.dll"
    ASLR enabled DEP disabled (0x40)

"d3dx9_35.dll"
    ASLR enabled DEP disabled (0x40)

"d3dx9_36.dll"
    ASLR enabled DEP disabled (0x40)

"D3DX9_37.dll"
    ASLR enabled DEP disabled (0x40)

"D3DX9_38.dll"
    ASLR enabled DEP disabled (0x40)

"D3DX9_39.dll"
    ASLR enabled DEP disabled (0x40)

"D3DX9_40.dll"
    ASLR enabled DEP disabled (0x40)

"D3DX9_41.dll"
    ASLR enabled DEP disabled (0x40)

"D3DX9_42.dll"
    ASLR enabled DEP disabled (0x40)

"xactengine2_3.dll"
    ASLR disabled DEP disabled (0x0)

"xactengine2_4.dll"
    ASLR disabled DEP disabled (0x0)

"xactengine2_5.dll"
    ASLR disabled DEP disabled (0x0)

"xactengine2_6.dll"
    ASLR disabled DEP disabled (0x0)

"xactengine2_7.dll"
    ASLR disabled DEP disabled (0x0)

"xactengine2_8.dll"
    ASLR disabled DEP disabled (0x0)

"xactengine2_9.dll"
    ASLR disabled DEP disabled (0x0)

"xactengine3_0.dll"
    ASLR enabled DEP disabled (0x40)

"xactengine3_1.dll"
    ASLR enabled DEP disabled (0x40)

"xactengine3_2.dll"
    ASLR enabled DEP disabled (0x40)

"xactengine3_3.dll"
    ASLR enabled DEP disabled (0x40)

"xactengine3_4.dll"
    ASLR enabled DEP disabled (0x40)

"xactengine3_5.dll"
    ASLR enabled DEP disabled (0x40)

"xactengine3_6.dll"
    ASLR enabled DEP disabled (0x40)

"d3dx10.dll"
    ASLR enabled DEP disabled (0x40)

"d3dx10_33.dll"
    ASLR enabled DEP disabled (0x40)

"d3dx10_34.dll"
    ASLR enabled DEP disabled (0x40)

"d3dx10_35.dll"
    ASLR enabled DEP disabled (0x40)

"d3dx10_36.dll"
    ASLR enabled DEP disabled (0x40)

"d3dx10_37.dll"
    ASLR enabled DEP disabled (0x40)

"d3dx10_38.dll"
    ASLR enabled DEP disabled (0x40)

"d3dx10_39.dll"
    ASLR enabled DEP disabled (0x40)

"d3dx10_40.dll"
    ASLR enabled DEP disabled (0x40)

"d3dx10_41.dll"
    ASLR enabled DEP disabled (0x40)

"d3dx10_42.dll"
    ASLR enabled DEP disabled (0x40)

"d3dx11_42.dll"
    ASLR enabled DEP disabled (0x40)

"d3dx9d_33.dll"
    ASLR enabled DEP disabled (0x40)

"d3dx9_24.dll"
    ASLR disabled DEP disabled (0x0)

"d3dx9_25.dll"
    ASLR disabled DEP disabled (0x0)

"d3dx9_26.dll"
    ASLR disabled DEP disabled (0x0)

"d3dx9_27.dll"
    ASLR disabled DEP disabled (0x0)

"d3dx9_28.dll"
    ASLR disabled DEP disabled (0x0)

"d3dx9_29.dll"
    ASLR disabled DEP disabled (0x0)

"XAPOFX1_0.dll"
    ASLR enabled DEP disabled (0x40)

"XAPOFX1_1.dll"
    ASLR enabled DEP disabled (0x40)

"XAPOFX1_2.dll"
    ASLR enabled DEP disabled (0x40)

"XAPOFX1_3.dll"
    ASLR enabled DEP disabled (0x40)

"XAPOFX1_4.dll"
    ASLR enabled DEP disabled (0x40)

"XAudio2_0.dll"
    ASLR enabled DEP disabled (0x40)

"XAudio2_1.dll"
    ASLR enabled DEP disabled (0x40)

"XAudio2_2.dll"
    ASLR enabled DEP disabled (0x40)

"XAudio2_3.dll"
    ASLR enabled DEP disabled (0x40)

"XAudio2_4.dll"
    ASLR enabled DEP disabled (0x40)

"XAudio2_5.dll"
    ASLR enabled DEP disabled (0x40)

"XAudio2_6.dll"
    ASLR enabled DEP disabled (0x40)

"xactengine2_0.dll"
    ASLR disabled DEP disabled (0x0)

"xactengine2_1.dll"
    ASLR disabled DEP disabled (0x0)

"xactengine2_10.dll"
    ASLR disabled DEP disabled (0x0)
```

## License

Licensed under the [MIT license](./LICENSE).
