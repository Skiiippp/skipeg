# skipeg
JPEG encoder and decoder in Rust

### Implmentation Notes
- Encodes from & decodes to a 24-bit PPM image
- Uses 4:2:0 chroma subsampling scheme

### Potential Optimizations
- Could use different system for reading PPM data (not BufRead or whatever)

### Random Thoughts
- Errors relating to RGB -> YCbCr seem to be concentrated around dark areas in the original image
