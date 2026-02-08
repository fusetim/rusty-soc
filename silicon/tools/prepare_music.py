"""
Small script to transform music files already sorted by "albums/artists" into the 
music and cover-art formats needed by our mini "MP3" player.

Usage:
    python prepare_music.py input_directory output_directory

The script will look for music files (e.g., .mp3, .wav) in the input subdirectories,
convert them to the required format, and save them in the output directory while maintaining the same 
subdirectory structure. It will also look for cover art images (.webp, .png, .jpg) in the same subdirectories 
and convert them to the required format (RGB565 128x128) for the mini "MP3" player. 

Input directory structure example:
```
input_directory/
├── Artist1/
│   |── Track1.mp3
│   |── Track1.webp
│   |── Track2.mp3
│   |── Track2.webp
├── Artist2/
│   |── Track1.mp3
│   |── Track1.png
│   |── Track2.mp3
│   |── Track2.png
```

Output directory structure example:
```
output_directory/
├── Artist1/
│   |── Track1/
│   |   |── music.raw
│   |   |── art.raw
│   |── Track2/
│   |   |── music.raw
│   |   |── art.raw
├── Artist2/
│   |── Track1/
│   |   |── music.raw
│   |   |── art.raw
"""

import os
import shutil
import subprocess
from img2rgb565 import convert_to_rgb565
from PIL.Image import Resampling
from PIL import Image
import argparse

def convert_music(input_path, output_path):
    """
    Convert a single music file to the required format and save it to the output path.
    """
    # Ensure the output parent directory exists
    os.makedirs(os.path.dirname(output_path), exist_ok=True)
    # Use ffmpeg to convert the music file to raw PCM format (8-bit, 48000 Hz, mono)
    command = [
        'ffmpeg',
        '-i', input_path,
        '-acodec', 'pcm_u8',
        '-f', 'u8',
        '-ar', '48000',
        '-ac', '1',
        '-y',  # Overwrite output file if it exists
        output_path
    ]
    try:
        subprocess.run(command, check=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
        print(f"Converted music: {input_path} -> {output_path}")
    except subprocess.CalledProcessError as e:
        print(f"Error converting music: {e.stderr.decode()}")
        return False
    return True

def convert_cover_art(input_path, output_path):
    """
    Convert a cover art image to RGB565 format and save it to the output path.
    """
    try:
        image = Image.open(input_path)
        print(f"Loaded cover art: {input_path} (size: {image.size[0]}x{image.size[1]})")
        # Resize the image to 128x128 and crop to center if necessary
        
        image = image.resize((128, 128), Resampling.LANCZOS)
        # Convert the image to RGB565 format
        rgb565_data = convert_to_rgb565(image)
        # Save the RGB565 data as a raw binary file
        with open(output_path, 'wb') as f:
            f.write(rgb565_data.tobytes())
        print(f"Converted cover art: {input_path} -> {output_path}")
    except Exception as e:
        print(f"Error converting cover art: {e}")
        return False
    return True

def transform_album(input_dir, output_dir):
    """
    Transform all music files and cover art in the input directory to the required formats and save them in the output directory.
    """
    for file in os.listdir(input_dir):
        input_path = os.path.join(input_dir, file)
        if os.path.isfile(input_path):
            filename, ext = os.path.splitext(file)
            # Quick recap of the expected output structure:
            # track.mp3 -> track/music.raw
            # track.webp -> track/art.raw
            if ext.lower() in ['.mp3', '.wav']:
                output_music_path = os.path.join(output_dir, filename, 'music.raw')
                convert_music(input_path, output_music_path)
            elif ext.lower() in ['.webp', '.png', '.jpg', '.jpeg']:
                output_art_path = os.path.join(output_dir, filename, 'art.raw')
                convert_cover_art(input_path, output_art_path)
    
def main():
    parser = argparse.ArgumentParser(description='Prepare music and cover art for the mini "MP3" player.')
    parser.add_argument('input_directory', type=str, help='Path to the input directory containing music and cover art files.')
    parser.add_argument('output_directory', type=str, help='Path to the output directory where converted files will be saved.')
    
    args = parser.parse_args()
    
    print(f"Input directory: {args.input_directory}")
    print(f"Output directory: {args.output_directory}")

    # Walk through the input directory and transform each album (subdirectory)
    for dir in os.listdir(args.input_directory):
        input_album_path = os.path.join(args.input_directory, dir)
        output_album_path = os.path.join(args.output_directory, dir)
        if os.path.isdir(input_album_path):
            print(f"Processing album: {dir}")
            transform_album(input_album_path, output_album_path)
        else:
            print(f"Skipping non-directory item: {input_album_path}")

if __name__ == "__main__":
    main()