# massif-visualizer

## install
```bash
sudo apt install massif-visualizer 
```

## run exe with 

```bash
valgrind --tool=massif target/debug/examples/vec_3
```
- should generate massif.out.<PID> file
  

## start   massif-visualizer

```bash
export DISPLAY=:1
massif-visualizer

# load the output file
```