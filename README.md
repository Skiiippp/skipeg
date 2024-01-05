# skipeg
JPEG encoder and decoder in Rust

### Progress
- Read PPM images
- 4:2:0 chroma subsampling
- DCT (in progress)

### Implmentation Notes
- Encodes from & decodes to a 24-bit PPM image
- Uses 4:2:0 chroma subsampling scheme

### Potential Optimizations
- Could use different system for reading PPM data (not BufRead or whatever)
- A lot of vector manipulation stuff is done via sequentially populating a new buffer, which prob isn't the fastest way to go (good for rapid dev and learning tho I guess)

### Random Thoughts
- Image of difference between subsampled and original image serves as a primitive kind of edge detection

