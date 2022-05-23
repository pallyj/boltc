use bolt_ext::{ExtensionHost, Operator, Fix, Precedence, declare_extension};

declare_extension!(register_extension);

#[allow(improper_ctypes_definitions)]
extern "C" fn register_extension(host: &mut dyn ExtensionHost) {
    host.register_operator(Operator { name: "nilCoalesce",
                                      symbol: "??",
                                      fix: Fix::Infix,
                                      precedence: Precedence::Comparison });
}