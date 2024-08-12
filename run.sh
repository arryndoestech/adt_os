qemu-system-aarch64 -smp 4 -m 1024 -cpu cortex-a76 -M virt -serial mon:stdio -nographic -device loader,addr=0x40800000,cpu-num=0,file=my_tut_os.o
