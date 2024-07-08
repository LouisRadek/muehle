cargo build --release
cd ../muehle-ui/target/debug
muehle.exe -w ../../../muehle/target/release/agent.exe -b ../../../muehle/target/release/agent.exe
cd ../../../muehle
