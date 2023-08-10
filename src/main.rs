fn main() {
    // get arguments from the cli
    let args: Vec<String> = std::env::args().collect();

    // write to a local file (relative to the executable) called "CLIArgsLogger.log" that contains the arguments
    std::fs::write("CLIArgsLogger.log", args.join("\n")).unwrap();
}
