#define rustg_udp_send(addr, text) call(RUST_G, "udp_send")(addr, text)
