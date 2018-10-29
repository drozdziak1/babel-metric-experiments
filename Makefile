OBJS := c_impl

CFLAGS += -lm

all: c-impl
	cargo build

c-impl:
	$(CC) $(CFLAGS) c_impl.c -o c_impl

clean:
	rm -rf $(OBJS)

mrproper: clean
	cargo clean
