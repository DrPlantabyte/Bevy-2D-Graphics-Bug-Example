# Bevy-2D-Graphics-Bug-Example
Minimum program to preduce graphics bug on some Linux computers

## The Problem
When running the program on some Linux computers, the following graphics bug is observed:

![Screenshot from 2022-09-04 12-23-00](https://user-images.githubusercontent.com/1922739/188294459-5c7ff94a-1ddf-44ef-85c3-74e9401c59b0.png)

Note the jagged artifacts around the sprite image. These artifacts appear around ALL 2D sprites on my system, and thresholding the alpha channel on the sprite does not fix it. When I run the program, Cargo and Bevy print the following text:

```bash
$ cargo run
warning: crate `Bevy_2D_Graphics_Bug_Example` should have a snake case name
  |
  = note: `#[warn(non_snake_case)]` on by default
  = help: convert the identifier to snake case: `bevy_2_d_graphics_bug_example`

warning: `Bevy-2D-Graphics-Bug-Example` (bin "Bevy-2D-Graphics-Bug-Example") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 0.15s
     Running `target/debug/Bevy-2D-Graphics-Bug-Example`
2022-09-04T02:26:01.624531Z  INFO winit::platform_impl::platform::x11::window: Guessed window scale factor: 1    
2022-09-04T02:26:01.694433Z  INFO bevy_render::renderer: AdapterInfo { name: "Intel(R) HD Graphics 630 (KBL GT2)", vendor: 32902, device: 22811, device_type: IntegratedGpu, backend: Vulkan }
```

### Hardware and Driver Information

```bash
$ uname -a
Linux pop-os 5.19.0-76051900-generic #202207312230~1660780566~22.04~9d60db1 SMP PREEMPT_DYNAMIC Thu A x86_64 x86_64 x86_64 GNU/Linux

$ sudo lspci -vnn | grep VGA -A 16
00:02.0 VGA compatible controller [0300]: Intel Corporation HD Graphics 630 [8086:591b] (rev 04) (prog-if 00 [VGA controller])
	Subsystem: CLEVO/KAPOK Computer HD Graphics 630 [1558:8509]
	Flags: bus master, fast devsel, latency 0, IRQ 129
	Memory at de000000 (64-bit, non-prefetchable) [size=16M]
	Memory at a0000000 (64-bit, prefetchable) [size=512M]
	I/O ports at f000 [size=64]
	Expansion ROM at 000c0000 [virtual] [disabled] [size=128K]
	Capabilities: [40] Vendor Specific Information: Len=0c <?>
	Capabilities: [70] Express Root Complex Integrated Endpoint, MSI 00
	Capabilities: [ac] MSI: Enable+ Count=1/1 Maskable- 64bit-
	Capabilities: [d0] Power Management version 2
	Capabilities: [100] Process Address Space ID (PASID)
	Capabilities: [200] Address Translation Service (ATS)
	Capabilities: [300] Page Request Interface (PRI)
	Kernel driver in use: i915
	Kernel modules: i915


$ glxinfo | grep OpenGL
OpenGL vendor string: Intel
OpenGL renderer string: Mesa Intel(R) HD Graphics 630 (KBL GT2)
OpenGL core profile version string: 4.6 (Core Profile) Mesa 22.0.5
OpenGL core profile shading language version string: 4.60
OpenGL core profile context flags: (none)
OpenGL core profile profile mask: core profile
OpenGL core profile extensions:
OpenGL version string: 4.6 (Compatibility Profile) Mesa 22.0.5
OpenGL shading language version string: 4.60
OpenGL context flags: (none)
OpenGL profile mask: compatibility profile
OpenGL extensions:
OpenGL ES profile version string: OpenGL ES 3.2 Mesa 22.0.5
OpenGL ES profile shading language version string: OpenGL ES GLSL ES 3.20
OpenGL ES profile extensions:


$ vulkaninfo | head -n 20
MESA-INTEL: warning: Performance support disabled, consider sysctl dev.i915.perf_stream_paranoid=0

WARNING: lavapipe is not a conformant vulkan implementation, testing use only.
==========
VULKANINFO
==========

Vulkan Instance Version: 1.3.204


Instance Extensions: count = 19
===============================
	VK_EXT_acquire_drm_display             : extension revision 1
	VK_EXT_acquire_xlib_display            : extension revision 1
	VK_EXT_debug_report                    : extension revision 10
	VK_EXT_debug_utils                     : extension revision 2
	VK_EXT_direct_mode_display             : extension revision 1
	VK_EXT_display_surface_counter         : extension revision 1
	VK_KHR_device_group_creation           : extension revision 1
	VK_KHR_display                         : extension revision 23
	VK_KHR_external_fence_capabilities     : extension revision 1
	VK_KHR_external_memory_capabilities    : extension revision 1
	VK_KHR_external_semaphore_capabilities : extension revision 1
WARNING: lavapipe is not a conformant vulkan implementation, testing use only.


$ sudo lshw -c video | grep configuration
       configuration: depth=32 driver=i915 latency=0 resolution=1920,1080


$ modinfo i915 | head -n 10
filename:       /lib/modules/5.19.0-76051900-generic/kernel/drivers/gpu/drm/i915/i915.ko
license:        GPL and additional rights
description:    Intel Graphics
author:         Intel Corporation
author:         Tungsten Graphics, Inc.
import_ns:      DMA_BUF
firmware:       i915/skl_huc_2.0.0.bin
firmware:       i915/bxt_huc_2.0.0.bin
firmware:       i915/kbl_huc_4.0.0.bin
firmware:       i915/glk_huc_4.0.0.bin
```

### More Hardware Details
```bash
$ sudo lshw
pop-os                      
    description: Notebook
    product: Gazelle (Not Applicable)
    vendor: System76
    version: gaze12
    serial: Not Applicable
    width: 64 bits
    capabilities: smbios-3.0.0 dmi-3.0.0 smp vsyscall32
    configuration: boot=normal chassis=notebook family=Not Applicable sku=Not Applicable uuid=4c5bfa80-db01-0000-0000-000000000000
  *-core
       description: Motherboard
       product: Gazelle
       vendor: System76
       physical id: 0
       version: gaze12
       serial: Not Applicable
       slot: Not Applicable
     *-firmware
          description: BIOS
          vendor: American Megatrends Inc.
          physical id: 0
          version: 1.05.15RSA-1
          date: 01/15/2018
          size: 64KiB
          capacity: 16MiB
          capabilities: pci upgrade shadowing cdboot bootselect socketedrom edd int5printscreen int9keyboard int17printer acpi usb biosbootspecification uefi
     *-memory
          description: System Memory
          physical id: 13
          slot: System board or motherboard
          size: 8GiB
        *-bank:0
             description: [empty]
             physical id: 0
             slot: ChannelA-DIMM0
        *-bank:1
             description: SODIMM DDR4 Synchronous 2400 MHz (0.4 ns)
             product: M471A1K43CB1-CRC
             vendor: Samsung
             physical id: 1
             serial: 20603E05
             slot: ChannelB-DIMM0
             size: 8GiB
             width: 64 bits
             clock: 2400MHz (0.4ns)
     *-cache:0
          description: L1 cache
          physical id: 17
          slot: L1 Cache
          size: 128KiB
          capacity: 128KiB
          capabilities: synchronous internal write-back unified
          configuration: level=1
     *-cache:1
          description: L2 cache
          physical id: 18
          slot: L2 Cache
          size: 512KiB
          capacity: 512KiB
          capabilities: synchronous internal write-back unified
          configuration: level=2
     *-cache:2
          description: L3 cache
          physical id: 19
          slot: L3 Cache
          size: 3MiB
          capacity: 3MiB
          capabilities: synchronous internal write-back unified
          configuration: level=3
     *-cpu
          description: CPU
          product: Intel(R) Core(TM) i3-7100H CPU @ 3.00GHz
          vendor: Intel Corp.
          physical id: 1a
          bus info: cpu@0
          version: 6.158.9
          serial: To Be Filled By O.E.M.
          slot: U3E1
          size: 3GHz
          capacity: 4005MHz
          width: 64 bits
          clock: 100MHz
          capabilities: lm fpu fpu_exception wp vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush dts acpi mmx fxsr sse sse2 ss ht tm pbe syscall nx pdpe1gb rdtscp x86-64 constant_tsc art arch_perfmon pebs bts rep_good nopl xtopology nonstop_tsc cpuid aperfmperf pni pclmulqdq dtes64 monitor ds_cpl vmx est tm2 ssse3 sdbg fma cx16 xtpr pdcm pcid sse4_1 sse4_2 x2apic movbe popcnt tsc_deadline_timer aes xsave avx f16c rdrand lahf_lm abm 3dnowprefetch cpuid_fault epb invpcid_single pti ssbd ibrs ibpb stibp tpr_shadow vnmi flexpriority ept vpid ept_ad fsgsbase tsc_adjust sgx bmi1 avx2 smep bmi2 erms invpcid mpx rdseed adx smap clflushopt intel_pt xsaveopt xsavec xgetbv1 xsaves dtherm arat pln pts hwp hwp_notify hwp_act_window hwp_epp md_clear flush_l1d arch_capabilities cpufreq
          configuration: cores=2 enabledcores=2 microcode=240 threads=4
     *-pci
          description: Host bridge
          product: Xeon E3-1200 v6/7th Gen Core Processor Host Bridge/DRAM Registers
          vendor: Intel Corporation
          physical id: 100
          bus info: pci@0000:00:00.0
          version: 05
          width: 32 bits
          clock: 33MHz
        *-display
             description: VGA compatible controller
             product: HD Graphics 630
             vendor: Intel Corporation
             physical id: 2
             bus info: pci@0000:00:02.0
             logical name: /dev/fb0
             version: 04
             width: 64 bits
             clock: 33MHz
             capabilities: pciexpress msi pm vga_controller bus_master cap_list rom fb
             configuration: depth=32 driver=i915 latency=0 resolution=1920,1080
             resources: irq:129 memory:de000000-deffffff memory:a0000000-bfffffff ioport:f000(size=64) memory:c0000-dffff
        *-usb
             description: USB controller
             product: 100 Series/C230 Series Chipset Family USB 3.0 xHCI Controller
             vendor: Intel Corporation
             physical id: 14
             bus info: pci@0000:00:14.0
             version: 31
             width: 64 bits
             clock: 33MHz
             capabilities: pm msi xhci bus_master cap_list
             configuration: driver=xhci_hcd latency=0
             resources: irq:125 memory:df210000-df21ffff
           *-usbhost:0
                product: xHCI Host Controller
                vendor: Linux 5.19.0-76051900-generic xhci-hcd
                physical id: 0
                bus info: usb@1
                logical name: usb1
                version: 5.19
                capabilities: usb-2.00
                configuration: driver=hub slots=16 speed=480Mbit/s
              *-usb:0
                   description: Video
                   product: Chicony USB 2.0 Camera: Chicony
                   vendor: SunplusIT Inc
                   physical id: 2
                   bus info: usb@1:2
                   logical name: input14
                   logical name: /dev/input/event7
                   version: 36.01
                   capabilities: usb-2.00 usb
                   configuration: driver=uvcvideo maxpower=500mA speed=480Mbit/s
              *-usb:1
                   description: Bluetooth wireless interface
                   product: Wireless-AC 3168 Bluetooth
                   vendor: Intel Corp.
                   physical id: 3
                   bus info: usb@1:3
                   version: 0.01
                   capabilities: bluetooth usb-2.00
                   configuration: driver=btusb maxpower=100mA speed=12Mbit/s
           *-usbhost:1
                product: xHCI Host Controller
                vendor: Linux 5.19.0-76051900-generic xhci-hcd
                physical id: 1
                bus info: usb@2
                logical name: usb2
                version: 5.19
                capabilities: usb-3.00
                configuration: driver=hub slots=8 speed=5000Mbit/s
        *-generic
             description: Signal processing controller
             product: 100 Series/C230 Series Chipset Family Thermal Subsystem
             vendor: Intel Corporation
             physical id: 14.2
             bus info: pci@0000:00:14.2
             version: 31
             width: 64 bits
             clock: 33MHz
             capabilities: pm msi cap_list
             configuration: driver=intel_pch_thermal latency=0
             resources: irq:18 memory:df22e000-df22efff
        *-communication
             description: Communication controller
             product: 100 Series/C230 Series Chipset Family MEI Controller #1
             vendor: Intel Corporation
             physical id: 16
             bus info: pci@0000:00:16.0
             version: 31
             width: 64 bits
             clock: 33MHz
             capabilities: pm msi bus_master cap_list
             configuration: driver=mei_me latency=0
             resources: irq:130 memory:df22d000-df22dfff
        *-sata
             description: SATA controller
             product: HM170/QM170 Chipset SATA Controller [AHCI Mode]
             vendor: Intel Corporation
             physical id: 17
             bus info: pci@0000:00:17.0
             logical name: scsi0
             version: 31
             width: 32 bits
             clock: 66MHz
             capabilities: sata msi pm ahci_1.0 bus_master cap_list emulated
             configuration: driver=ahci latency=0
             resources: irq:126 memory:df228000-df229fff memory:df22c000-df22c0ff ioport:f090(size=8) ioport:f080(size=4) ioport:f060(size=32) memory:df22b000-df22b7ff
           *-disk
                description: ATA Disk
                product: Samsung SSD 850
                physical id: 0.0.0
                bus info: scsi@0:0.0.0
                logical name: /dev/sda
                version: 1B6Q
                serial: S33CNX0J652708X
                size: 232GiB (250GB)
                capabilities: gpt-1.00 partitioned partitioned:gpt
                configuration: ansiversion=5 guid=d683ac20-3cfc-4a15-92cd-a54ecd54f99c logicalsectorsize=512 sectorsize=512
              *-volume:0
                   description: Windows FAT volume
                   vendor: mkfs.fat
                   physical id: 1
                   bus info: scsi@0:0.0.0,1
                   logical name: /dev/sda1
                   logical name: /boot/efi
                   version: FAT32
                   serial: 1eae-a3d8
                   size: 495MiB
                   capacity: 497MiB
                   capabilities: boot fat initialized
                   configuration: FATs=2 filesystem=fat mount.fstype=vfat mount.options=rw,relatime,fmask=0077,dmask=0077,codepage=437,iocharset=iso8859-1,shortname=mixed,errors=remount-ro state=mounted
              *-volume:1
                   description: Windows FAT volume
                   vendor: mkfs.fat
                   physical id: 2
                   bus info: scsi@0:0.0.0,2
                   logical name: /dev/sda2
                   logical name: /recovery
                   version: FAT32
                   serial: 1eae-b885
                   size: 4075MiB
                   capacity: 4095MiB
                   capabilities: fat initialized
                   configuration: FATs=2 filesystem=fat mount.fstype=vfat mount.options=rw,relatime,fmask=0077,dmask=0077,codepage=437,iocharset=iso8859-1,shortname=mixed,errors=remount-ro name=recovery state=mounted
              *-volume:2
                   description: EFI partition
                   physical id: 3
                   bus info: scsi@0:0.0.0,3
                   logical name: /dev/sda3
                   serial: 05149b65-ca1c-4339-ab2e-2a3669cbf8c0
                   size: 224GiB
                   capacity: 224GiB
                   width: 3142253776 bits
                   capabilities: encrypted luks initialized
                   configuration: bits=33207024848 filesystem=luks hash=sha256 version=2
              *-volume:3
                   description: Linux swap volume
                   vendor: Linux
                   physical id: 4
                   bus info: scsi@0:0.0.0,4
                   logical name: /dev/sda4
                   version: 1
                   serial: 0bbca874-d913-4132-99a5-da95ec3e327b
                   size: 4095MiB
                   capacity: 4095MiB
                   capabilities: nofs nomount swap initialized
                   configuration: filesystem=swap pagesize=4096
        *-pci:0
             description: PCI bridge
             product: 100 Series/C230 Series Chipset Family PCI Express Root Port #1
             vendor: Intel Corporation
             physical id: 1c
             bus info: pci@0000:00:1c.0
             version: f1
             width: 32 bits
             clock: 33MHz
             capabilities: pci pciexpress msi pm normal_decode bus_master cap_list
             configuration: driver=pcieport
             resources: irq:122 ioport:2000(size=4096) memory:7cc00000-7cdfffff ioport:7ce00000(size=2097152)
        *-pci:1
             description: PCI bridge
             product: 100 Series/C230 Series Chipset Family PCI Express Root Port #5
             vendor: Intel Corporation
             physical id: 1c.4
             bus info: pci@0000:00:1c.4
             version: f1
             width: 32 bits
             clock: 33MHz
             capabilities: pci pciexpress msi pm normal_decode bus_master cap_list
             configuration: driver=pcieport
             resources: irq:123 ioport:e000(size=4096) memory:df100000-df1fffff
           *-generic
                description: MMC Host
                product: RTL8411B PCI Express Card Reader
                vendor: Realtek Semiconductor Co., Ltd.
                physical id: 0
                bus info: pci@0000:02:00.0
                logical name: mmc0
                version: 01
                width: 32 bits
                clock: 33MHz
                capabilities: pm msi pciexpress msix vpd bus_master cap_list rom
                configuration: driver=rtsx_pci latency=0
                resources: irq:127 memory:df115000-df115fff memory:df100000-df10ffff
           *-network
                description: Ethernet interface
                product: RTL8111/8168/8411 PCI Express Gigabit Ethernet Controller
                vendor: Realtek Semiconductor Co., Ltd.
                physical id: 0.1
                bus info: pci@0000:02:00.1
                logical name: enp2s0f1
                version: 12
                serial: 80:fa:5b:4c:01:db
                capacity: 1Gbit/s
                width: 64 bits
                clock: 33MHz
                capabilities: pm msi pciexpress msix vpd bus_master cap_list ethernet physical tp mii 10bt 10bt-fd 100bt 100bt-fd 1000bt-fd autonegotiation
                configuration: autonegotiation=on broadcast=yes driver=r8169 driverversion=5.19.0-76051900-generic firmware=rtl8411-2_0.0.1 07/08/13 latency=0 link=no multicast=yes port=twisted pair
                resources: irq:16 ioport:e000(size=256) memory:df114000-df114fff memory:df110000-df113fff
        *-pci:2
             description: PCI bridge
             product: 100 Series/C230 Series Chipset Family PCI Express Root Port #7
             vendor: Intel Corporation
             physical id: 1c.6
             bus info: pci@0000:00:1c.6
             version: f1
             width: 32 bits
             clock: 33MHz
             capabilities: pci pciexpress msi pm normal_decode bus_master cap_list
             configuration: driver=pcieport
             resources: irq:124 memory:df000000-df0fffff
           *-network
                description: Wireless interface
                product: Dual Band Wireless-AC 3168NGW [Stone Peak]
                vendor: Intel Corporation
                physical id: 0
                bus info: pci@0000:03:00.0
                logical name: wlp3s0
                version: 10
                serial: f4:96:34:8c:fb:ae
                width: 64 bits
                clock: 33MHz
                capabilities: pm msi pciexpress bus_master cap_list ethernet physical wireless
                configuration: broadcast=yes driver=iwlwifi driverversion=5.19.0-76051900-generic firmware=29.198743027.0 3168-29.ucode ip=192.168.0.8 latency=0 link=yes multicast=yes wireless=IEEE 802.11
                resources: irq:131 memory:df000000-df001fff
        *-isa
             description: ISA bridge
             product: HM175 Chipset LPC/eSPI Controller
             vendor: Intel Corporation
             physical id: 1f
             bus info: pci@0000:00:1f.0
             version: 31
             width: 32 bits
             clock: 33MHz
             capabilities: isa bus_master
             configuration: latency=0
           *-pnp00:00
                product: PnP device PNP0c02
                physical id: 0
                capabilities: pnp
                configuration: driver=system
           *-pnp00:01
                product: PnP device PNP0c02
                physical id: 1
                capabilities: pnp
                configuration: driver=system
           *-pnp00:02
                product: PnP device PNP0b00
                physical id: 2
                capabilities: pnp
                configuration: driver=rtc_cmos
           *-pnp00:03
                product: PnP device INT3f0d
                physical id: 3
                capabilities: pnp
                configuration: driver=system
           *-pnp00:04
                product: PnP device PNP0303
                physical id: 4
                capabilities: pnp
                configuration: driver=i8042 kbd
           *-pnp00:05
                product: PnP device SYN1222
                physical id: 5
                capabilities: pnp
                configuration: driver=i8042 aux
           *-pnp00:06
                product: PnP device PNP0c02
                physical id: 6
                capabilities: pnp
                configuration: driver=system
           *-pnp00:07
                product: PnP device PNP0c02
                physical id: 7
                capabilities: pnp
                configuration: driver=system
           *-pnp00:08
                product: PnP device PNP0c02
                physical id: 8
                capabilities: pnp
                configuration: driver=system
           *-pnp00:09
                product: PnP device PNP0c02
                physical id: 9
                capabilities: pnp
                configuration: driver=system
        *-memory UNCLAIMED
             description: Memory controller
             product: 100 Series/C230 Series Chipset Family Power Management Controller
             vendor: Intel Corporation
             physical id: 1f.2
             bus info: pci@0000:00:1f.2
             version: 31
             width: 32 bits
             clock: 33MHz (30.3ns)
             configuration: latency=0
             resources: memory:df224000-df227fff
        *-multimedia
             description: Audio device
             product: CM238 HD Audio Controller
             vendor: Intel Corporation
             physical id: 1f.3
             bus info: pci@0000:00:1f.3
             logical name: card0
             logical name: /dev/snd/controlC0
             logical name: /dev/snd/hwC0D0
             logical name: /dev/snd/hwC0D2
             logical name: /dev/snd/pcmC0D0c
             logical name: /dev/snd/pcmC0D0p
             logical name: /dev/snd/pcmC0D10p
             logical name: /dev/snd/pcmC0D3p
             logical name: /dev/snd/pcmC0D7p
             logical name: /dev/snd/pcmC0D8p
             logical name: /dev/snd/pcmC0D9p
             version: 31
             width: 64 bits
             clock: 33MHz
             capabilities: pm msi bus_master cap_list
             configuration: driver=snd_hda_intel latency=32
             resources: irq:132 memory:df220000-df223fff memory:df200000-df20ffff
           *-input:0
                product: HDA Intel PCH Mic
                physical id: 0
                logical name: input15
                logical name: /dev/input/event8
           *-input:1
                product: HDA Intel PCH Front Headphone
                physical id: 1
                logical name: input16
                logical name: /dev/input/event9
           *-input:2
                product: HDA Intel PCH HDMI/DP,pcm=3
                physical id: 2
                logical name: input17
                logical name: /dev/input/event10
           *-input:3
                product: HDA Intel PCH HDMI/DP,pcm=7
                physical id: 3
                logical name: input18
                logical name: /dev/input/event11
           *-input:4
                product: HDA Intel PCH HDMI/DP,pcm=8
                physical id: 4
                logical name: input19
                logical name: /dev/input/event12
           *-input:5
                product: HDA Intel PCH HDMI/DP,pcm=9
                physical id: 5
                logical name: input20
                logical name: /dev/input/event13
           *-input:6
                product: HDA Intel PCH HDMI/DP,pcm=10
                physical id: 6
                logical name: input21
                logical name: /dev/input/event14
        *-serial
             description: SMBus
             product: 100 Series/C230 Series Chipset Family SMBus
             vendor: Intel Corporation
             physical id: 1f.4
             bus info: pci@0000:00:1f.4
             version: 31
             width: 64 bits
             clock: 33MHz
             configuration: driver=i801_smbus latency=0
             resources: irq:16 memory:df22a000-df22a0ff ioport:f040(size=32)
  *-input:0
       product: Power Button
       physical id: 1
       logical name: input0
       logical name: /dev/input/event0
       capabilities: platform
  *-input:1
       product: Sleep Button
       physical id: 2
       logical name: input1
       logical name: /dev/input/event1
       capabilities: platform
  *-input:2
       product: SynPS/2 Synaptics TouchPad
       physical id: 3
       logical name: input10
       logical name: /dev/input/event6
       logical name: /dev/input/mouse0
       capabilities: i8042
  *-input:3
       product: Video Bus
       physical id: 4
       logical name: input11
       logical name: /dev/input/event5
       capabilities: platform
  *-input:4
       product: Lid Switch
       physical id: 5
       logical name: input2
       logical name: /dev/input/event2
       capabilities: platform
  *-input:5
       product: Power Button
       physical id: 6
       logical name: input3
       logical name: /dev/input/event3
       capabilities: platform
  *-input:6
       product: AT Translated Set 2 keyboard
       physical id: 7
       logical name: input4
       logical name: /dev/input/event4
       logical name: input4::capslock
       logical name: input4::numlock
       logical name: input4::scrolllock
       capabilities: i8042


$ modinfo i915
filename:       /lib/modules/5.19.0-76051900-generic/kernel/drivers/gpu/drm/i915/i915.ko
license:        GPL and additional rights
description:    Intel Graphics
author:         Intel Corporation
author:         Tungsten Graphics, Inc.
import_ns:      DMA_BUF
firmware:       i915/skl_huc_2.0.0.bin
firmware:       i915/bxt_huc_2.0.0.bin
firmware:       i915/kbl_huc_4.0.0.bin
firmware:       i915/glk_huc_4.0.0.bin
firmware:       i915/kbl_huc_4.0.0.bin
firmware:       i915/kbl_huc_4.0.0.bin
firmware:       i915/cml_huc_4.0.0.bin
firmware:       i915/icl_huc_9.0.0.bin
firmware:       i915/ehl_huc_9.0.0.bin
firmware:       i915/ehl_huc_9.0.0.bin
firmware:       i915/tgl_huc_7.9.3.bin
firmware:       i915/tgl_huc_7.9.3.bin
firmware:       i915/dg1_huc_7.9.3.bin
firmware:       i915/tgl_huc_7.9.3.bin
firmware:       i915/tgl_huc_7.9.3.bin
firmware:       i915/tgl_guc_69.0.3.bin
firmware:       i915/adlp_guc_69.0.3.bin
firmware:       i915/skl_guc_70.1.1.bin
firmware:       i915/bxt_guc_70.1.1.bin
firmware:       i915/kbl_guc_70.1.1.bin
firmware:       i915/glk_guc_70.1.1.bin
firmware:       i915/kbl_guc_70.1.1.bin
firmware:       i915/kbl_guc_70.1.1.bin
firmware:       i915/cml_guc_70.1.1.bin
firmware:       i915/icl_guc_70.1.1.bin
firmware:       i915/ehl_guc_70.1.1.bin
firmware:       i915/ehl_guc_70.1.1.bin
firmware:       i915/tgl_guc_70.1.1.bin
firmware:       i915/tgl_guc_70.1.1.bin
firmware:       i915/dg1_guc_70.1.1.bin
firmware:       i915/tgl_guc_70.1.1.bin
firmware:       i915/adlp_guc_70.1.1.bin
firmware:       i915/dg2_guc_70.1.2.bin
firmware:       i915/bxt_dmc_ver1_07.bin
firmware:       i915/skl_dmc_ver1_27.bin
firmware:       i915/kbl_dmc_ver1_04.bin
firmware:       i915/glk_dmc_ver1_04.bin
firmware:       i915/icl_dmc_ver1_09.bin
firmware:       i915/tgl_dmc_ver2_12.bin
firmware:       i915/rkl_dmc_ver2_03.bin
firmware:       i915/dg1_dmc_ver2_02.bin
firmware:       i915/adls_dmc_ver2_01.bin
firmware:       i915/adlp_dmc_ver2_16.bin
srcversion:     001A3FA4725B6461A07F22E
alias:          pci:v00008086d000056B2sv*sd*bc03sc*i*
alias:          pci:v00008086d00005697sv*sd*bc03sc*i*
alias:          pci:v00008086d00005696sv*sd*bc03sc*i*
alias:          pci:v00008086d000056B0sv*sd*bc03sc*i*
alias:          pci:v00008086d00005695sv*sd*bc03sc*i*
alias:          pci:v00008086d00005694sv*sd*bc03sc*i*
alias:          pci:v00008086d00005693sv*sd*bc03sc*i*
alias:          pci:v00008086d00005692sv*sd*bc03sc*i*
alias:          pci:v00008086d00005691sv*sd*bc03sc*i*
alias:          pci:v00008086d00005690sv*sd*bc03sc*i*
alias:          pci:v00008086d0000A7A9sv*sd*bc03sc*i*
alias:          pci:v00008086d0000A7A8sv*sd*bc03sc*i*
alias:          pci:v00008086d0000A7A1sv*sd*bc03sc*i*
alias:          pci:v00008086d0000A7A0sv*sd*bc03sc*i*
alias:          pci:v00008086d0000A721sv*sd*bc03sc*i*
alias:          pci:v00008086d0000A720sv*sd*bc03sc*i*
alias:          pci:v00008086d0000A78Bsv*sd*bc03sc*i*
alias:          pci:v00008086d0000A78Asv*sd*bc03sc*i*
alias:          pci:v00008086d0000A789sv*sd*bc03sc*i*
alias:          pci:v00008086d0000A788sv*sd*bc03sc*i*
alias:          pci:v00008086d0000A783sv*sd*bc03sc*i*
alias:          pci:v00008086d0000A782sv*sd*bc03sc*i*
alias:          pci:v00008086d0000A781sv*sd*bc03sc*i*
alias:          pci:v00008086d0000A780sv*sd*bc03sc*i*
alias:          pci:v00008086d00004909sv*sd*bc03sc*i*
alias:          pci:v00008086d00004908sv*sd*bc03sc*i*
alias:          pci:v00008086d00004907sv*sd*bc03sc*i*
alias:          pci:v00008086d00004906sv*sd*bc03sc*i*
alias:          pci:v00008086d00004905sv*sd*bc03sc*i*
alias:          pci:v00008086d000046D2sv*sd*bc03sc*i*
alias:          pci:v00008086d000046D1sv*sd*bc03sc*i*
alias:          pci:v00008086d000046D0sv*sd*bc03sc*i*
alias:          pci:v00008086d000046C3sv*sd*bc03sc*i*
alias:          pci:v00008086d000046C2sv*sd*bc03sc*i*
alias:          pci:v00008086d000046C1sv*sd*bc03sc*i*
alias:          pci:v00008086d000046C0sv*sd*bc03sc*i*
alias:          pci:v00008086d000046B3sv*sd*bc03sc*i*
alias:          pci:v00008086d000046B2sv*sd*bc03sc*i*
alias:          pci:v00008086d000046B1sv*sd*bc03sc*i*
alias:          pci:v00008086d000046B0sv*sd*bc03sc*i*
alias:          pci:v00008086d00004628sv*sd*bc03sc*i*
alias:          pci:v00008086d00004626sv*sd*bc03sc*i*
alias:          pci:v00008086d0000462Asv*sd*bc03sc*i*
alias:          pci:v00008086d000046AAsv*sd*bc03sc*i*
alias:          pci:v00008086d000046A8sv*sd*bc03sc*i*
alias:          pci:v00008086d000046A6sv*sd*bc03sc*i*
alias:          pci:v00008086d000046A3sv*sd*bc03sc*i*
alias:          pci:v00008086d000046A2sv*sd*bc03sc*i*
alias:          pci:v00008086d000046A1sv*sd*bc03sc*i*
alias:          pci:v00008086d000046A0sv*sd*bc03sc*i*
alias:          pci:v00008086d00004693sv*sd*bc03sc*i*
alias:          pci:v00008086d00004692sv*sd*bc03sc*i*
alias:          pci:v00008086d00004690sv*sd*bc03sc*i*
alias:          pci:v00008086d0000468Asv*sd*bc03sc*i*
alias:          pci:v00008086d00004688sv*sd*bc03sc*i*
alias:          pci:v00008086d00004682sv*sd*bc03sc*i*
alias:          pci:v00008086d00004680sv*sd*bc03sc*i*
alias:          pci:v00008086d00004C9Asv*sd*bc03sc*i*
alias:          pci:v00008086d00004C90sv*sd*bc03sc*i*
alias:          pci:v00008086d00004C8Csv*sd*bc03sc*i*
alias:          pci:v00008086d00004C8Bsv*sd*bc03sc*i*
alias:          pci:v00008086d00004C8Asv*sd*bc03sc*i*
alias:          pci:v00008086d00004C80sv*sd*bc03sc*i*
alias:          pci:v00008086d00009AF8sv*sd*bc03sc*i*
alias:          pci:v00008086d00009AD9sv*sd*bc03sc*i*
alias:          pci:v00008086d00009AC9sv*sd*bc03sc*i*
alias:          pci:v00008086d00009AC0sv*sd*bc03sc*i*
alias:          pci:v00008086d00009A78sv*sd*bc03sc*i*
alias:          pci:v00008086d00009A59sv*sd*bc03sc*i*
alias:          pci:v00008086d00009A49sv*sd*bc03sc*i*
alias:          pci:v00008086d00009A40sv*sd*bc03sc*i*
alias:          pci:v00008086d00009A70sv*sd*bc03sc*i*
alias:          pci:v00008086d00009A68sv*sd*bc03sc*i*
alias:          pci:v00008086d00009A60sv*sd*bc03sc*i*
alias:          pci:v00008086d00004E71sv*sd*bc03sc*i*
alias:          pci:v00008086d00004E61sv*sd*bc03sc*i*
alias:          pci:v00008086d00004E57sv*sd*bc03sc*i*
alias:          pci:v00008086d00004E55sv*sd*bc03sc*i*
alias:          pci:v00008086d00004E51sv*sd*bc03sc*i*
alias:          pci:v00008086d00004571sv*sd*bc03sc*i*
alias:          pci:v00008086d00004557sv*sd*bc03sc*i*
alias:          pci:v00008086d00004555sv*sd*bc03sc*i*
alias:          pci:v00008086d00004551sv*sd*bc03sc*i*
alias:          pci:v00008086d00004541sv*sd*bc03sc*i*
alias:          pci:v00008086d00008A5Dsv*sd*bc03sc*i*
alias:          pci:v00008086d00008A51sv*sd*bc03sc*i*
alias:          pci:v00008086d00008A71sv*sd*bc03sc*i*
alias:          pci:v00008086d00008A70sv*sd*bc03sc*i*
alias:          pci:v00008086d00008A5Csv*sd*bc03sc*i*
alias:          pci:v00008086d00008A5Bsv*sd*bc03sc*i*
alias:          pci:v00008086d00008A5Asv*sd*bc03sc*i*
alias:          pci:v00008086d00008A59sv*sd*bc03sc*i*
alias:          pci:v00008086d00008A58sv*sd*bc03sc*i*
alias:          pci:v00008086d00008A57sv*sd*bc03sc*i*
alias:          pci:v00008086d00008A56sv*sd*bc03sc*i*
alias:          pci:v00008086d00008A54sv*sd*bc03sc*i*
alias:          pci:v00008086d00008A53sv*sd*bc03sc*i*
alias:          pci:v00008086d00008A52sv*sd*bc03sc*i*
alias:          pci:v00008086d00008A50sv*sd*bc03sc*i*
alias:          pci:v00008086d00009BCCsv*sd*bc03sc*i*
alias:          pci:v00008086d00009BCAsv*sd*bc03sc*i*
alias:          pci:v00008086d00009B41sv*sd*bc03sc*i*
alias:          pci:v00008086d00009BACsv*sd*bc03sc*i*
alias:          pci:v00008086d00009BAAsv*sd*bc03sc*i*
alias:          pci:v00008086d00009B21sv*sd*bc03sc*i*
alias:          pci:v00008086d00009BF6sv*sd*bc03sc*i*
alias:          pci:v00008086d00009BE6sv*sd*bc03sc*i*
alias:          pci:v00008086d00009BC8sv*sd*bc03sc*i*
alias:          pci:v00008086d00009BC6sv*sd*bc03sc*i*
alias:          pci:v00008086d00009BC5sv*sd*bc03sc*i*
alias:          pci:v00008086d00009BC4sv*sd*bc03sc*i*
alias:          pci:v00008086d00009BC2sv*sd*bc03sc*i*
alias:          pci:v00008086d00009BA8sv*sd*bc03sc*i*
alias:          pci:v00008086d00009BA5sv*sd*bc03sc*i*
alias:          pci:v00008086d00009BA4sv*sd*bc03sc*i*
alias:          pci:v00008086d00009BA2sv*sd*bc03sc*i*
alias:          pci:v00008086d00003EA2sv*sd*bc03sc*i*
alias:          pci:v00008086d000087CAsv*sd*bc03sc*i*
alias:          pci:v00008086d00003EA3sv*sd*bc03sc*i*
alias:          pci:v00008086d00003EA0sv*sd*bc03sc*i*
alias:          pci:v00008086d00003EA4sv*sd*bc03sc*i*
alias:          pci:v00008086d00003EA1sv*sd*bc03sc*i*
alias:          pci:v00008086d00003EA8sv*sd*bc03sc*i*
alias:          pci:v00008086d00003EA7sv*sd*bc03sc*i*
alias:          pci:v00008086d00003EA6sv*sd*bc03sc*i*
alias:          pci:v00008086d00003EA5sv*sd*bc03sc*i*
alias:          pci:v00008086d00003EA9sv*sd*bc03sc*i*
alias:          pci:v00008086d00003E9Bsv*sd*bc03sc*i*
alias:          pci:v00008086d00003E94sv*sd*bc03sc*i*
alias:          pci:v00008086d00003E9Csv*sd*bc03sc*i*
alias:          pci:v00008086d00003E9Asv*sd*bc03sc*i*
alias:          pci:v00008086d00003E98sv*sd*bc03sc*i*
alias:          pci:v00008086d00003E96sv*sd*bc03sc*i*
alias:          pci:v00008086d00003E92sv*sd*bc03sc*i*
alias:          pci:v00008086d00003E91sv*sd*bc03sc*i*
alias:          pci:v00008086d00003E99sv*sd*bc03sc*i*
alias:          pci:v00008086d00003E93sv*sd*bc03sc*i*
alias:          pci:v00008086d00003E90sv*sd*bc03sc*i*
alias:          pci:v00008086d000087C0sv*sd*bc03sc*i*
alias:          pci:v00008086d0000591Csv*sd*bc03sc*i*
alias:          pci:v00008086d0000593Bsv*sd*bc03sc*i*
alias:          pci:v00008086d00005927sv*sd*bc03sc*i*
alias:          pci:v00008086d00005923sv*sd*bc03sc*i*
alias:          pci:v00008086d00005926sv*sd*bc03sc*i*
alias:          pci:v00008086d0000591Dsv*sd*bc03sc*i*
alias:          pci:v00008086d0000591Bsv*sd*bc03sc*i*
alias:          pci:v00008086d0000591Asv*sd*bc03sc*i*
alias:          pci:v00008086d00005917sv*sd*bc03sc*i*
alias:          pci:v00008086d00005912sv*sd*bc03sc*i*
alias:          pci:v00008086d0000591Esv*sd*bc03sc*i*
alias:          pci:v00008086d00005921sv*sd*bc03sc*i*
alias:          pci:v00008086d00005916sv*sd*bc03sc*i*
alias:          pci:v00008086d0000590Bsv*sd*bc03sc*i*
alias:          pci:v00008086d0000590Asv*sd*bc03sc*i*
alias:          pci:v00008086d00005908sv*sd*bc03sc*i*
alias:          pci:v00008086d00005902sv*sd*bc03sc*i*
alias:          pci:v00008086d00005915sv*sd*bc03sc*i*
alias:          pci:v00008086d0000590Esv*sd*bc03sc*i*
alias:          pci:v00008086d00005913sv*sd*bc03sc*i*
alias:          pci:v00008086d00005906sv*sd*bc03sc*i*
alias:          pci:v00008086d00003185sv*sd*bc03sc*i*
alias:          pci:v00008086d00003184sv*sd*bc03sc*i*
alias:          pci:v00008086d00005A85sv*sd*bc03sc*i*
alias:          pci:v00008086d00005A84sv*sd*bc03sc*i*
alias:          pci:v00008086d00001A85sv*sd*bc03sc*i*
alias:          pci:v00008086d00001A84sv*sd*bc03sc*i*
alias:          pci:v00008086d00000A84sv*sd*bc03sc*i*
alias:          pci:v00008086d0000193Dsv*sd*bc03sc*i*
alias:          pci:v00008086d0000193Bsv*sd*bc03sc*i*
alias:          pci:v00008086d0000193Asv*sd*bc03sc*i*
alias:          pci:v00008086d00001932sv*sd*bc03sc*i*
alias:          pci:v00008086d0000192Dsv*sd*bc03sc*i*
alias:          pci:v00008086d0000192Bsv*sd*bc03sc*i*
alias:          pci:v00008086d0000192Asv*sd*bc03sc*i*
alias:          pci:v00008086d00001927sv*sd*bc03sc*i*
alias:          pci:v00008086d00001926sv*sd*bc03sc*i*
alias:          pci:v00008086d00001923sv*sd*bc03sc*i*
alias:          pci:v00008086d0000191Dsv*sd*bc03sc*i*
alias:          pci:v00008086d0000191Bsv*sd*bc03sc*i*
alias:          pci:v00008086d0000191Asv*sd*bc03sc*i*
alias:          pci:v00008086d00001912sv*sd*bc03sc*i*
alias:          pci:v00008086d0000191Esv*sd*bc03sc*i*
alias:          pci:v00008086d00001921sv*sd*bc03sc*i*
alias:          pci:v00008086d00001916sv*sd*bc03sc*i*
alias:          pci:v00008086d00001917sv*sd*bc03sc*i*
alias:          pci:v00008086d0000190Bsv*sd*bc03sc*i*
alias:          pci:v00008086d0000190Asv*sd*bc03sc*i*
alias:          pci:v00008086d00001902sv*sd*bc03sc*i*
alias:          pci:v00008086d00001915sv*sd*bc03sc*i*
alias:          pci:v00008086d0000190Esv*sd*bc03sc*i*
alias:          pci:v00008086d00001913sv*sd*bc03sc*i*
alias:          pci:v00008086d00001906sv*sd*bc03sc*i*
alias:          pci:v00008086d000022B3sv*sd*bc03sc*i*
alias:          pci:v00008086d000022B2sv*sd*bc03sc*i*
alias:          pci:v00008086d000022B1sv*sd*bc03sc*i*
alias:          pci:v00008086d000022B0sv*sd*bc03sc*i*
alias:          pci:v00008086d0000163Dsv*sd*bc03sc*i*
alias:          pci:v00008086d0000163Asv*sd*bc03sc*i*
alias:          pci:v00008086d00001632sv*sd*bc03sc*i*
alias:          pci:v00008086d0000163Esv*sd*bc03sc*i*
alias:          pci:v00008086d0000163Bsv*sd*bc03sc*i*
alias:          pci:v00008086d00001636sv*sd*bc03sc*i*
alias:          pci:v00008086d0000162Dsv*sd*bc03sc*i*
alias:          pci:v00008086d0000162Asv*sd*bc03sc*i*
alias:          pci:v00008086d00001622sv*sd*bc03sc*i*
alias:          pci:v00008086d0000162Esv*sd*bc03sc*i*
alias:          pci:v00008086d0000162Bsv*sd*bc03sc*i*
alias:          pci:v00008086d00001626sv*sd*bc03sc*i*
alias:          pci:v00008086d0000161Dsv*sd*bc03sc*i*
alias:          pci:v00008086d0000161Asv*sd*bc03sc*i*
alias:          pci:v00008086d00001612sv*sd*bc03sc*i*
alias:          pci:v00008086d0000161Esv*sd*bc03sc*i*
alias:          pci:v00008086d0000161Bsv*sd*bc03sc*i*
alias:          pci:v00008086d00001616sv*sd*bc03sc*i*
alias:          pci:v00008086d0000160Dsv*sd*bc03sc*i*
alias:          pci:v00008086d0000160Asv*sd*bc03sc*i*
alias:          pci:v00008086d00001602sv*sd*bc03sc*i*
alias:          pci:v00008086d0000160Esv*sd*bc03sc*i*
alias:          pci:v00008086d0000160Bsv*sd*bc03sc*i*
alias:          pci:v00008086d00001606sv*sd*bc03sc*i*
alias:          pci:v00008086d00000F33sv*sd*bc03sc*i*
alias:          pci:v00008086d00000F32sv*sd*bc03sc*i*
alias:          pci:v00008086d00000F31sv*sd*bc03sc*i*
alias:          pci:v00008086d00000F30sv*sd*bc03sc*i*
alias:          pci:v00008086d00000D2Esv*sd*bc03sc*i*
alias:          pci:v00008086d00000D2Bsv*sd*bc03sc*i*
alias:          pci:v00008086d00000D2Asv*sd*bc03sc*i*
alias:          pci:v00008086d00000D26sv*sd*bc03sc*i*
alias:          pci:v00008086d00000D22sv*sd*bc03sc*i*
alias:          pci:v00008086d00000C2Esv*sd*bc03sc*i*
alias:          pci:v00008086d00000C2Bsv*sd*bc03sc*i*
alias:          pci:v00008086d00000C2Asv*sd*bc03sc*i*
alias:          pci:v00008086d00000C26sv*sd*bc03sc*i*
alias:          pci:v00008086d00000C22sv*sd*bc03sc*i*
alias:          pci:v00008086d0000042Esv*sd*bc03sc*i*
alias:          pci:v00008086d0000042Bsv*sd*bc03sc*i*
alias:          pci:v00008086d0000042Asv*sd*bc03sc*i*
alias:          pci:v00008086d00000426sv*sd*bc03sc*i*
alias:          pci:v00008086d00000422sv*sd*bc03sc*i*
alias:          pci:v00008086d00000A2Esv*sd*bc03sc*i*
alias:          pci:v00008086d00000A2Bsv*sd*bc03sc*i*
alias:          pci:v00008086d00000A2Asv*sd*bc03sc*i*
alias:          pci:v00008086d00000A26sv*sd*bc03sc*i*
alias:          pci:v00008086d00000A22sv*sd*bc03sc*i*
alias:          pci:v00008086d00000D1Esv*sd*bc03sc*i*
alias:          pci:v00008086d00000D1Bsv*sd*bc03sc*i*
alias:          pci:v00008086d00000D1Asv*sd*bc03sc*i*
alias:          pci:v00008086d00000D16sv*sd*bc03sc*i*
alias:          pci:v00008086d00000D12sv*sd*bc03sc*i*
alias:          pci:v00008086d00000C1Esv*sd*bc03sc*i*
alias:          pci:v00008086d00000C1Bsv*sd*bc03sc*i*
alias:          pci:v00008086d00000C1Asv*sd*bc03sc*i*
alias:          pci:v00008086d00000C16sv*sd*bc03sc*i*
alias:          pci:v00008086d00000C12sv*sd*bc03sc*i*
alias:          pci:v00008086d0000041Esv*sd*bc03sc*i*
alias:          pci:v00008086d0000041Bsv*sd*bc03sc*i*
alias:          pci:v00008086d0000041Asv*sd*bc03sc*i*
alias:          pci:v00008086d00000416sv*sd*bc03sc*i*
alias:          pci:v00008086d00000412sv*sd*bc03sc*i*
alias:          pci:v00008086d00000A1Esv*sd*bc03sc*i*
alias:          pci:v00008086d00000A1Bsv*sd*bc03sc*i*
alias:          pci:v00008086d00000A1Asv*sd*bc03sc*i*
alias:          pci:v00008086d00000A16sv*sd*bc03sc*i*
alias:          pci:v00008086d00000A12sv*sd*bc03sc*i*
alias:          pci:v00008086d00000D0Esv*sd*bc03sc*i*
alias:          pci:v00008086d00000D0Bsv*sd*bc03sc*i*
alias:          pci:v00008086d00000D0Asv*sd*bc03sc*i*
alias:          pci:v00008086d00000D06sv*sd*bc03sc*i*
alias:          pci:v00008086d00000D02sv*sd*bc03sc*i*
alias:          pci:v00008086d00000C0Esv*sd*bc03sc*i*
alias:          pci:v00008086d00000C0Bsv*sd*bc03sc*i*
alias:          pci:v00008086d00000C0Asv*sd*bc03sc*i*
alias:          pci:v00008086d00000C06sv*sd*bc03sc*i*
alias:          pci:v00008086d00000C02sv*sd*bc03sc*i*
alias:          pci:v00008086d0000040Esv*sd*bc03sc*i*
alias:          pci:v00008086d0000040Bsv*sd*bc03sc*i*
alias:          pci:v00008086d0000040Asv*sd*bc03sc*i*
alias:          pci:v00008086d00000406sv*sd*bc03sc*i*
alias:          pci:v00008086d00000402sv*sd*bc03sc*i*
alias:          pci:v00008086d00000A0Esv*sd*bc03sc*i*
alias:          pci:v00008086d00000A0Bsv*sd*bc03sc*i*
alias:          pci:v00008086d00000A0Asv*sd*bc03sc*i*
alias:          pci:v00008086d00000A06sv*sd*bc03sc*i*
alias:          pci:v00008086d00000A02sv*sd*bc03sc*i*
alias:          pci:v00008086d0000016Asv*sd*bc03sc*i*
alias:          pci:v00008086d00000162sv*sd*bc03sc*i*
alias:          pci:v00008086d0000015Asv*sd*bc03sc*i*
alias:          pci:v00008086d00000152sv*sd*bc03sc*i*
alias:          pci:v00008086d00000166sv*sd*bc03sc*i*
alias:          pci:v00008086d00000156sv*sd*bc03sc*i*
alias:          pci:v00008086d0000016Asv0000152Dsd00008990bc03sc*i*
alias:          pci:v00008086d00000126sv*sd*bc03sc*i*
alias:          pci:v00008086d00000116sv*sd*bc03sc*i*
alias:          pci:v00008086d00000106sv*sd*bc03sc*i*
alias:          pci:v00008086d00000122sv*sd*bc03sc*i*
alias:          pci:v00008086d00000112sv*sd*bc03sc*i*
alias:          pci:v00008086d0000010Asv*sd*bc03sc*i*
alias:          pci:v00008086d00000102sv*sd*bc03sc*i*
alias:          pci:v00008086d00000046sv*sd*bc03sc*i*
alias:          pci:v00008086d00000042sv*sd*bc03sc*i*
alias:          pci:v00008086d0000A011sv*sd*bc03sc*i*
alias:          pci:v00008086d0000A001sv*sd*bc03sc*i*
alias:          pci:v00008086d00002E92sv*sd*bc03sc*i*
alias:          pci:v00008086d00002E42sv*sd*bc03sc*i*
alias:          pci:v00008086d00002E32sv*sd*bc03sc*i*
alias:          pci:v00008086d00002E22sv*sd*bc03sc*i*
alias:          pci:v00008086d00002E12sv*sd*bc03sc*i*
alias:          pci:v00008086d00002E02sv*sd*bc03sc*i*
alias:          pci:v00008086d00002A42sv*sd*bc03sc*i*
alias:          pci:v00008086d00002A12sv*sd*bc03sc*i*
alias:          pci:v00008086d00002A02sv*sd*bc03sc*i*
alias:          pci:v00008086d000029D2sv*sd*bc03sc*i*
alias:          pci:v00008086d000029C2sv*sd*bc03sc*i*
alias:          pci:v00008086d000029B2sv*sd*bc03sc*i*
alias:          pci:v00008086d000029A2sv*sd*bc03sc*i*
alias:          pci:v00008086d00002992sv*sd*bc03sc*i*
alias:          pci:v00008086d00002982sv*sd*bc03sc*i*
alias:          pci:v00008086d00002972sv*sd*bc03sc*i*
alias:          pci:v00008086d000027AEsv*sd*bc03sc*i*
alias:          pci:v00008086d000027A2sv*sd*bc03sc*i*
alias:          pci:v00008086d00002772sv*sd*bc03sc*i*
alias:          pci:v00008086d00002592sv*sd*bc03sc*i*
alias:          pci:v00008086d0000258Asv*sd*bc03sc*i*
alias:          pci:v00008086d00002582sv*sd*bc03sc*i*
alias:          pci:v00008086d00002572sv*sd*bc03sc*i*
alias:          pci:v00008086d0000358Esv*sd*bc03sc*i*
alias:          pci:v00008086d00003582sv*sd*bc03sc*i*
alias:          pci:v00008086d00002562sv*sd*bc03sc*i*
alias:          pci:v00008086d00003577sv*sd*bc03sc*i*
depends:        ttm,drm,drm_display_helper,drm_kms_helper,video,cec,drm_buddy,i2c-algo-bit
retpoline:      Y
intree:         Y
name:           i915
vermagic:       5.19.0-76051900-generic SMP preempt mod_unload modversions 
sig_id:         PKCS#7
signer:         Build time autogenerated kernel key
sig_key:        38:A6:08:1F:27:D9:5D:37:7F:A3:62:03:0E:D5:7A:21:D5:36:AF:1D
sig_hashalgo:   sha512
signature:      40:26:48:B7:4A:47:FC:14:2A:5D:7C:D3:D6:FF:F3:84:56:B5:93:53:
		FC:DC:F1:BA:54:6C:D8:88:65:3C:09:03:EA:2D:63:55:02:3C:53:09:
		39:4D:6B:14:73:7A:8C:5C:40:13:C9:AD:54:79:C0:41:8D:F4:A0:2D:
		A0:5C:36:AD:6F:F6:35:0A:01:79:82:07:33:33:91:87:D2:EF:03:D9:
		F5:6D:DF:F1:51:DE:7C:4E:C9:FA:5E:77:08:8C:8B:9E:32:0B:DA:3F:
		9F:0F:75:1A:1A:6D:C1:9B:04:D8:68:38:75:BE:E1:73:F5:B8:FF:57:
		4E:5E:A3:A0:D8:E9:7F:39:05:06:7E:A3:50:62:AD:33:DB:92:CE:B8:
		00:A5:EE:DF:FA:FC:9A:D2:27:D7:3E:99:51:F2:81:F4:9D:7E:74:E5:
		6B:66:63:13:93:E9:18:38:35:5C:D4:A0:B0:22:6A:A7:1D:18:18:EC:
		5F:A5:DC:32:61:0D:DC:31:19:BE:5D:72:B9:87:16:5F:E9:49:D1:81:
		14:D9:9E:63:91:AE:B4:4A:8C:7C:10:FB:31:BD:3D:32:FA:0E:BC:B2:
		5D:85:CE:8D:DB:7F:0E:E0:06:37:68:22:F8:CE:F0:D0:5E:BE:A5:73:
		C1:BB:DE:A1:66:75:8E:D1:90:B2:A4:CE:FC:BD:08:4F:43:B1:B5:8F:
		0F:EB:7F:32:FA:36:85:6B:C2:50:CB:6F:8F:CF:FD:BA:72:3D:3F:0F:
		AC:CA:5F:E3:D9:D0:D9:EE:ED:12:51:67:CA:AD:7E:A4:7B:80:39:5B:
		43:D7:6A:FC:8B:74:73:C6:5D:C2:E0:15:72:9D:F7:F2:DC:1D:98:92:
		D4:F3:E0:E2:A3:57:6A:56:A5:C1:2F:70:5E:A7:38:38:64:98:AE:45:
		95:15:C0:72:18:E5:4C:F6:A5:CB:06:A3:91:3E:05:C8:A6:67:E9:30:
		CB:28:CA:79:89:F8:3A:79:ED:FF:06:0F:22:B0:34:9A:79:B3:6D:54:
		B8:CF:30:3D:0B:2B:0D:79:7A:D7:52:E4:21:7C:4D:17:43:E1:A8:D7:
		8C:FC:60:41:8B:CB:4A:5A:B6:46:A7:CC:3C:23:B9:EF:CC:57:C8:F0:
		F2:24:CB:CC:DB:17:5A:91:ED:00:A5:25:5A:70:E8:50:7A:BC:B2:E0:
		60:1F:89:20:E6:94:4C:F8:43:A1:6E:8F:93:28:77:47:94:3D:D4:65:
		E3:A9:5D:C1:71:9B:2F:81:9C:94:48:6B:65:ED:50:D9:78:FE:B6:D9:
		68:2F:5B:0A:97:25:1A:37:30:A7:A9:83:B2:F3:44:1F:94:57:33:8F:
		5D:01:5A:23:16:F3:7B:B6:E7:22:CC:06
parm:           modeset:Use kernel modesetting [KMS] (0=disable, 1=on, -1=force vga console preference [default]) (int)
parm:           enable_dc:Enable power-saving display C-states. (-1=auto [default]; 0=disable; 1=up to DC5; 2=up to DC6; 3=up to DC5 with DC3CO; 4=up to DC6 with DC3CO) (int)
parm:           enable_fbc:Enable frame buffer compression for power savings (default: -1 (use per-chip default)) (int)
parm:           lvds_channel_mode:Specify LVDS channel mode (0=probe BIOS [default], 1=single-channel, 2=dual-channel) (int)
parm:           panel_use_ssc:Use Spread Spectrum Clock with panels [LVDS/eDP] (default: auto from VBT) (int)
parm:           vbt_sdvo_panel_type:Override/Ignore selection of SDVO panel mode in the VBT (-2=ignore, -1=auto [default], index in VBT BIOS table) (int)
parm:           reset:Attempt GPU resets (0=disabled, 1=full gpu reset, 2=engine reset [default]) (uint)
parm:           vbt_firmware:Load VBT from specified file under /lib/firmware (charp)
parm:           error_capture:Record the GPU state following a hang. This information in /sys/class/drm/card<N>/error is vital for triaging and debugging hangs. (bool)
parm:           enable_hangcheck:Periodically check GPU activity for detecting hangs. WARNING: Disabling this can cause system wide hangs. (default: true) (bool)
parm:           enable_psr:Enable PSR (0=disabled, 1=enable up to PSR1, 2=enable up to PSR2) Default: -1 (use per-chip default) (int)
parm:           psr_safest_params:Replace PSR VBT parameters by the safest and not optimal ones. This is helpful to detect if PSR issues are related to bad values set in  VBT. (0=use VBT parameters, 1=use safest parameters) (bool)
parm:           enable_psr2_sel_fetch:Enable PSR2 selective fetch (0=disabled, 1=enabled) Default: 0 (bool)
parm:           force_probe:Force probe the driver for specified devices. See CONFIG_DRM_I915_FORCE_PROBE for details. (charp)
parm:           disable_power_well:Disable display power wells when possible (-1=auto [default], 0=power wells always on, 1=power wells disabled when possible) (int)
parm:           enable_ips:Enable IPS (default: true) (int)
parm:           fastboot:Try to skip unnecessary mode sets at boot time (0=disabled, 1=enabled) Default: -1 (use per-chip default) (int)
parm:           load_detect_test:Force-enable the VGA load detect code for testing (default:false). For developers only. (bool)
parm:           force_reset_modeset_test:Force a modeset during gpu reset for testing (default:false). For developers only. (bool)
parm:           invert_brightness:Invert backlight brightness (-1 force normal, 0 machine defaults, 1 force inversion), please report PCI device ID, subsystem vendor and subsystem device ID to dri-devel@lists.freedesktop.org, if your machine needs it. It will then be included in an upcoming module version. (int)
parm:           disable_display:Disable display (default: false) (bool)
parm:           memtest:Perform a read/write test of all device memory on module load (default: off) (bool)
parm:           mmio_debug:Enable the MMIO debug code for the first N failures (default: off). This may negatively affect performance. (int)
parm:           verbose_state_checks:Enable verbose logs (ie. WARN_ON()) in case of unexpected hw state conditions. (bool)
parm:           nuclear_pageflip:Force enable atomic functionality on platforms that don't have full support yet. (bool)
parm:           edp_vswing:Ignore/Override vswing pre-emph table selection from VBT (0=use value from vbt [default], 1=low power swing(200mV),2=default swing(400mV)) (int)
parm:           enable_guc:Enable GuC load for GuC submission and/or HuC load. Required functionality can be selected using bitmask values. (-1=auto [default], 0=disable, 1=GuC submission, 2=HuC load) (int)
parm:           guc_log_level:GuC firmware logging level. Requires GuC to be loaded. (-1=auto [default], 0=disable, 1..4=enable with verbosity min..max) (int)
parm:           guc_firmware_path:GuC firmware path to use instead of the default one (charp)
parm:           huc_firmware_path:HuC firmware path to use instead of the default one (charp)
parm:           dmc_firmware_path:DMC firmware path to use instead of the default one (charp)
parm:           enable_dp_mst:Enable multi-stream transport (MST) for new DisplayPort sinks. (default: true) (bool)
parm:           enable_dpcd_backlight:Enable support for DPCD backlight control(-1=use per-VBT LFP backlight type setting [default], 0=disabled, 1=enable, 2=force VESA interface, 3=force Intel interface) (int)
parm:           enable_gvt:Enable support for Intel GVT-g graphics virtualization host support(default:false) (bool)
parm:           request_timeout_ms:Default request/fence/batch buffer expiration timeout. (uint)
parm:           lmem_size:Set the lmem size(in MiB) for each region. (default: 0, all memory) (uint)
parm:           mitigations:Selectively enable security mitigations for all Intel® GPUs in the system.

  auto -- enables all mitigations required for the platform [default]
  off  -- disables all mitigations

Individual mitigations can be enabled by passing a comma-separated string,
e.g. mitigations=residuals to enable only clearing residuals or
mitigations=auto,noresiduals to disable only the clear residual mitigation.
Either '!' or 'no' may be used to switch from enabling the mitigation to
disabling it.

Active mitigations for Ivybridge, Baytrail, Haswell:
  residuals -- clear all thread-local registers between contexts


```
