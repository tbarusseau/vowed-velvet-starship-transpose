cc_import(
    name = "libcomplex_polynomials",
    hdrs = ["inc/rust_api.h"],
    interface_library = "lib/libcomplex_polynomials.lib",
    shared_library = "lib/libcomplex_polynomials.dll",
)

cc_binary(
    name = "c-caller",
    deps = ["libcomplex_polynomials"],
    srcs = ["src/main.c", "inc/rust_api.h"],
    copts = ["-Iinc"],
)
