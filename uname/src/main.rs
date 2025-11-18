use clap::Parser;
use miaulib::get_platform_info;

/// Print certain system information. With no OPTION, same as -s.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct UnameArgs {
    /// print all information, in the following order,
    ///   except omit -p and -i if unknown:
    #[arg(short, long, verbatim_doc_comment)]
    all: bool,

    /// print the kernel name
    #[arg(short = 's', long)]
    kernel_name: bool,

    /// print the network node hostname
    #[arg(short, long)]
    nodename: bool,

    /// print the kernel release
    #[arg(short = 'r', long)]
    kernel_release: bool,

    /// print the kernel version
    #[arg(short = 'v', long)]
    kernel_version: bool,

    /// print the machine hardware name
    #[arg(short, long)]
    machine: bool,

    /// print the processor type (non-portable)
    #[arg(short, long)]
    processor: bool,

    /// print the hardware platform (non-portable)
    #[arg(short = 'i', long)]
    hardware_platform: bool,

    /// print the operating system
    #[arg(short, long)]
    operating_system: bool,
}

fn main() {
    let mut args = UnameArgs::parse();

    let no_flags_provided = ![
        args.all,
        args.kernel_name,
        args.nodename,
        args.kernel_release,
        args.kernel_version,
        args.machine,
        args.processor,
        args.hardware_platform,
        args.operating_system,
    ]
    .iter()
    .any(|&x| x);

    if no_flags_provided {
        args.kernel_name = true;
    }

    match get_platform_info() {
        Ok(info) => {
            let mut output_parts = Vec::new();

            if args.all {
                // -p and -i are omitted to match GNU uname behavior
                output_parts.push(info.sys_name);
                output_parts.push(info.node_name);
                output_parts.push(info.release);
                output_parts.push(info.version);
                output_parts.push(info.machine);
                if !cfg!(target_os = "macos") {
                    output_parts.push(info.os);
                }
            } else {
                if args.kernel_name {
                    output_parts.push(info.sys_name);
                }
                if args.nodename {
                    output_parts.push(info.node_name);
                }
                if args.kernel_release {
                    output_parts.push(info.release);
                }
                if args.kernel_version {
                    output_parts.push(info.version);
                }
                if args.machine {
                    output_parts.push(info.machine);
                }
                if args.processor {
                    output_parts.push("unknown".to_string());
                }
                if args.hardware_platform {
                    output_parts.push("unknown".to_string());
                }
                if args.operating_system {
                    output_parts.push(info.os);
                }
            }

            println!("{}", output_parts.join(" "))
        }
        Err(e) => {
            eprintln!("uname: error: {}", e);
            std::process::exit(1);
        }
    }
}
