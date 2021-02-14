target := "thumbv7em-none-eabihf"
mode := "release"
build-path := "./target/" + target + "/" + mode + "/"
proj := "stm32h7-try"
elf := build-path + proj
asm := build-path + proj + ".asm"

objdump := "rust-objdump"

flash: build
    @cd {{proj}} && cargo flash --{{mode}} --chip STM32H750VBTx --target {{target}}

build: firmware

debug: build
    @cd probe-debug && cargo run

firmware:
    @cd {{proj}} && cargo build --{{mode}} && cd ..
    @{{objdump}} -D {{elf}} | less > {{asm}}

clean:
    @rm -rf {{build-path}}