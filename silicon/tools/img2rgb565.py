#!/usr/bin/env python3
"""
Simple script to easily convert an image to RGB565 format for our application.

Usage:
    python img2rgb565.py input_image.png output_image.raw
    python img2rgb565.py -w 128 -h 128 input_image.png output_image.raw

Additional options:
    -w, --width: Specify the width of the output image (default: none, will use the input image width)
    -h, --height: Specify the height of the output image (default: none, will use the input image height)
    If image ratio can be preserved, the image will be resized to fit within the specified dimensions while 
    maintaining the aspect ratio. If not, a warn will be issued and the image will be resized to the 
    specified dimensions without preserving the aspect ratio.
"""
from PIL.Image import Resampling
from PIL import Image
import argparse
import numpy as np

def convert_to_rgb565(image):
    # Convert the image to RGB format
    image = image.convert('RGB')
    
    # Get the pixel data as a numpy array
    pixel_data = np.array(image)
    rgb565 = np.zeros((pixel_data.shape[0], pixel_data.shape[1], 2), dtype=np.uint8)
    
    # Convert RGB888 to RGB565
    for i in range(pixel_data.shape[0]):
        for j in range(pixel_data.shape[1]):
            r, g, b = pixel_data[i, j]
            #print(f"Pixel ({i}, {j}): R={r}, G={g}, B={b}", end=' ')
            r = int((r >> 3) & 0b11111 ) # 5 bits for red
            g = int((g >> 2) & 0b111111)  # 6 bits for green
            b = int((b >> 3) & 0b11111 ) # 5 bits for blue
            #print(f"-> R5={r} G6={g} B5={b}", end=' ')
            c = (r << 11)
            c += (g << 5)
            c += b
            #print(f"-> RGB565={c}")
            rgb565[i, j, 0] = (c >> 8) & 0xFF  # High byte
            rgb565[i, j, 1] = c & 0xFF         # Low byte
    return rgb565

def main():
    parser = argparse.ArgumentParser(description='Convert an image to RGB565 format.', add_help=False)
    parser.add_argument('input_image', type=str, help='Path to the input image file (e.g., PNG, JPEG).')
    parser.add_argument('output_file', type=str, help='Path to the output binary file (e.g., .raw).')
    parser.add_argument('-w', '--width', type=int, help='Width of the output image (default: input image width).')
    parser.add_argument('-h', '--height', type=int, help='Height of the output image (default: input image height).')
    
    args = parser.parse_args()
    
    # Load the input image
    try:
        image = Image.open(args.input_image)
        print(f"Loaded image: {args.input_image} (size: {image.size[0]}x{image.size[1]})")
    except Exception as e:
        print(f"Error loading image: {e}")
        return
    
    # Resize the image if width and height are specified
    if args.width and args.height:
        original_width, original_height = image.size
        aspect_ratio = original_width / original_height
        target_aspect_ratio = args.width / args.height
    elif args.width:
        # Only width specified, calculate height to maintain aspect ratio
        original_width, original_height = image.size
        aspect_ratio = original_width / original_height
        target_aspect_ratio = args.width / (args.width / aspect_ratio)
        args.height = int(args.width / aspect_ratio)
    elif args.height:
        # Only height specified, calculate width to maintain aspect ratio
        original_width, original_height = image.size
        aspect_ratio = original_width / original_height
        target_aspect_ratio = (args.height * aspect_ratio) / args.height
        args.width = int(args.height * aspect_ratio)

    if args.width and args.height:
        if aspect_ratio == target_aspect_ratio:
            # Aspect ratio is preserved, resize directly
            image = image.resize((args.width, args.height), Resampling.NEAREST)
        else:
            print("Warning: The specified dimensions do not preserve the aspect ratio. The image will be resized without preserving the aspect ratio.")
            image = image.resize((args.width, args.height), Resampling.NEAREST)
    
    # Convert the image to RGB565 format
    rgb565_data = convert_to_rgb565(image)
    
    # Save the RGB565 data to a binary file
    try:
        with open(args.output_file, 'wb') as f:
            f.write(rgb565_data.tobytes())
        if args.width and args.height:
            print(f"Saved RGB565 data to: {args.output_file} (size: {args.width}x{args.height} = {rgb565_data.nbytes} bytes)")
        else:
            print(f"Saved RGB565 data to: {args.output_file} (size:  {image.size[0]}x{image.size[1]} = {rgb565_data.nbytes} bytes)")
    except Exception as e:
        print(f"Error saving output file: {e}")

if __name__ == "__main__":
    main()