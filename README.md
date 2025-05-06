# CPU-Pegger
This tool allows you to stress test hardware by running an endless operation on CPU core(s) of your choosing.

## Compiling
To build this app, you'll need to install rust and then run this command using cargo:
```
cargo build --release
```

## Usage
To use, simply run the tool with the argument -c and the core number (0 based)
```
./cpu-pegger -c 0
```

You can also specify multiple cores:
```
./cpu-pegger -c 0 -c 1 -c 2
```

By default, the operation runs for 60 seconds, you can override this with the -d argument. The following command will peg core 0 for 10 seconds.
```
./cpu-pegger -c 0 -d 10
```

If you want to partially peg a core, you can do so via the -p command and provide a percentage. Default is 100. The following core pegs core 0 for 10 seconds at 50%
```
./cpu-pegger -c 0 -d 10 -p 50
```

