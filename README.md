# Offline Media Converter (GUI)

**Offline Media Converter** is a desktop application that enables the conversion of video and image files between various formats. It uses **FFmpeg** for media processing and supports **GPU-accelerated** conversion through **OpenCL**. The app has a simple **GTK**-based graphical user interface for easy interaction.

---

## Features

- **Video & Image Conversion**: Convert between various formats (e.g., MP4, AVI, JPG, PNG).
- **GPU Acceleration**: Uses **OpenCL** for hardware-accelerated processing, supporting various GPU vendors (NVIDIA, AMD, Intel).
- **Cross-Platform**: Works on **Windows**, **macOS**, and **Linux**.
- **Batch Processing**: Convert multiple files simultaneously.
- **Simple GUI**: Easy-to-use interface built with **GTK**.

---

## Prerequisites

- **OpenCL SDK**: Install the relevant OpenCL drivers for your GPU (NVIDIA, AMD, Intel).
- **FFmpeg**: Ensure FFmpeg is compiled with **OpenCL** support.

---

## Installation

1. **Install FFmpeg with OpenCL Support**: Follow the FFmpeg installation guide to enable OpenCL.
2. **Install OpenCL SDK**: Install the appropriate OpenCL SDK for your GPU.
3. **Download & Install the Application**: Clone the repository and build the GTK GUI app.

---

## Usage

1. **Launch the App**: Open the Offline Media Converter GUI.
2. **Select Files**: Choose input video/image files.
3. **Choose Output Format**: Select the desired output format (e.g., MP4, JPG).
4. **Configure Options**: Adjust settings like resolution or codec.
5. **Start Conversion**: Press the “Start” button to begin conversion with GPU acceleration.
6. **Track Progress**: Monitor conversion progress via the GUI.

